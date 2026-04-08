use crate::config::PostgresConfig;
use crate::error::{DbError, DbResult};
use crate::models::{
    CrawlTarget, DiscoveredLink, Document, DocumentContent, DocumentTerm, LinkCategory, Term,
};
use crate::pool::connect_and_initialize;
use crate::queries::{contents, crawl_targets, documents, links, terms};
use crate::schema::validate_schema_name;
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SearchEngineRepository {
    pool: PgPool,
    schema: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredDocument {
    pub crawl_target_id: i64,
    pub document_id: i64,
    pub indexed_terms: usize,
}

impl SearchEngineRepository {
    pub fn new(pool: PgPool, schema: impl Into<String>) -> DbResult<Self> {
        let schema = schema.into();
        validate_schema_name(&schema)?;
        Ok(Self { pool, schema })
    }

    pub async fn initialize(config: &PostgresConfig) -> DbResult<Self> {
        let pool = connect_and_initialize(config).await?;
        Self::new(pool, config.schema.clone())
    }

    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn upsert_crawl_target(&self, target: &CrawlTarget) -> DbResult<i64> {
        let mut tx = self.pool.begin().await?;
        let target_id = self.upsert_crawl_target_in_tx(&mut tx, target).await?;
        tx.commit().await?;
        Ok(target_id)
    }

    pub async fn store_indexed_document(
        &self,
        canonical_url: &str,
        plain_text: &str,
        term_frequency: &HashMap<String, usize>,
        extracted_links_count: usize,
    ) -> DbResult<StoredDocument> {
        let mut tx = self.pool.begin().await?;

        let crawl_target = CrawlTarget::new(canonical_url)?;
        let crawl_target_id = self.upsert_crawl_target_in_tx(&mut tx, &crawl_target).await?;

        let mut document = Document::new(canonical_url)?;
        document.crawl_target_id = Some(crawl_target_id);
        document.content_length = i64::try_from(plain_text.len()).map_err(|_| {
            DbError::Validation("document content length exceeds i64 range".to_string())
        })?;

        let document_id = self.upsert_document_in_tx(&mut tx, &document).await?;
        let content = DocumentContent::new(
            document_id,
            plain_text,
            i32::try_from(extracted_links_count).map_err(|_| {
                DbError::Validation("extracted links count exceeds i32 range".to_string())
            })?,
        )?;
        self.upsert_document_content_in_tx(&mut tx, &content).await?;

        let indexed_terms = self.replace_document_terms_in_tx(&mut tx, document_id, term_frequency)
            .await?;

        tx.commit().await?;

        Ok(StoredDocument {
            crawl_target_id,
            document_id,
            indexed_terms,
        })
    }

    pub async fn record_discovered_links(
        &self,
        source_document_id: Option<i64>,
        discovered_links: &[DiscoveredLink],
    ) -> DbResult<usize> {
        let mut tx = self.pool.begin().await?;
        let sql = links::insert_discovered_link_sql(&self.schema)?;
        let mut stored = 0usize;

        for discovered_link in discovered_links {
            let crawl_target_id = match discovered_link.category {
                LinkCategory::Visitable => {
                    let crawl_target = CrawlTarget::new(discovered_link.url.clone())?;
                    Some(self.upsert_crawl_target_in_tx(&mut tx, &crawl_target).await?)
                }
                LinkCategory::Junk => None,
            };

            sqlx::query(&sql)
                .bind(source_document_id)
                .bind(crawl_target_id)
                .bind(&discovered_link.url)
                .bind(discovered_link.category.as_str())
                .bind(&discovered_link.anchor_text)
                .bind(discovered_link.depth)
                .execute(&mut *tx)
                .await?;

            stored += 1;
        }

        tx.commit().await?;
        Ok(stored)
    }

    async fn upsert_crawl_target_in_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        target: &CrawlTarget,
    ) -> DbResult<i64> {
        let sql = crawl_targets::upsert_crawl_target_sql(&self.schema)?;

        let target_id = sqlx::query_scalar::<_, i64>(&sql)
            .bind(&target.canonical_url)
            .bind(&target.scheme)
            .bind(&target.host)
            .bind(&target.path)
            .bind(target.status.as_str())
            .bind(target.depth)
            .bind(target.priority)
            .bind(target.retry_count)
            .fetch_one(&mut *tx)
            .await?;

        Ok(target_id)
    }

    async fn upsert_document_in_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        document: &Document,
    ) -> DbResult<i64> {
        let sql = documents::upsert_document_sql(&self.schema)?;

        let document_id = sqlx::query_scalar::<_, i64>(&sql)
            .bind(document.crawl_target_id)
            .bind(&document.canonical_url)
            .bind(&document.scheme)
            .bind(&document.host)
            .bind(&document.path)
            .bind(&document.title)
            .bind(&document.content_type)
            .bind(document.http_status)
            .bind(&document.etag)
            .bind(document.content_length)
            .bind(&document.checksum)
            .bind(&document.language)
            .fetch_one(&mut *tx)
            .await?;

        Ok(document_id)
    }

    async fn upsert_document_content_in_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        content: &DocumentContent,
    ) -> DbResult<()> {
        let sql = contents::upsert_document_content_sql(&self.schema)?;

        sqlx::query(&sql)
            .bind(content.document_id)
            .bind(&content.raw_html)
            .bind(&content.plain_text)
            .bind(content.extracted_links_count)
            .execute(&mut *tx)
            .await?;

        Ok(())
    }

    async fn replace_document_terms_in_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        document_id: i64,
        term_frequency: &HashMap<String, usize>,
    ) -> DbResult<usize> {
        let delete_sql = terms::delete_document_terms_sql(&self.schema)?;
        let upsert_term_sql = terms::upsert_term_sql(&self.schema)?;
        let insert_document_term_sql = terms::insert_document_term_sql(&self.schema)?;

        sqlx::query(&delete_sql)
            .bind(document_id)
            .execute(&mut *tx)
            .await?;

        let mut indexed_terms = 0usize;

        for (term_value, frequency) in term_frequency {
            let term = Term::new(term_value.clone())?;
            let term_id = sqlx::query_scalar::<_, i64>(&upsert_term_sql)
                .bind(&term.term)
                .fetch_one(&mut *tx)
                .await?;
            let document_term = DocumentTerm::new(
                document_id,
                term_id,
                i32::try_from(*frequency).map_err(|_| {
                    DbError::Validation("term frequency exceeds i32 range".to_string())
                })?,
            )?;

            sqlx::query(&insert_document_term_sql)
                .bind(document_term.document_id)
                .bind(document_term.term_id)
                .bind(document_term.term_frequency)
                .execute(&mut *tx)
                .await?;

            indexed_terms += 1;
        }

        Ok(indexed_terms)
    }
}
