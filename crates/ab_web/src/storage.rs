use crate::error::AppError;
use crate::models::{ContentDetail, ContentListItem, summarize_text};
use db::schema::validate_schema_name;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};

#[derive(Clone)]
pub struct ContentStore {
    pool: PgPool,
    schema: String,
}

impl ContentStore {
    pub fn new(
        database_url: impl Into<String>,
        schema: impl Into<String>,
    ) -> Result<Self, AppError> {
        let database_url = database_url.into().trim().to_string();
        if database_url.is_empty() {
            return Err(AppError::config("DATABASE_URL cannot be empty"));
        }

        let schema = schema.into();
        validate_schema_name(&schema).map_err(|error| AppError::config(error.to_string()))?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .min_connections(0)
            .connect_lazy(&database_url)?;

        Ok(Self { pool, schema })
    }

    pub async fn latest_documents(&self, limit: u32) -> Result<Vec<ContentListItem>, AppError> {
        let sql = format!(
            "SELECT \
                d.id, \
                d.crawl_target_id, \
                d.canonical_url, \
                d.host, \
                d.path, \
                d.title, \
                d.content_type, \
                d.http_status, \
                d.language, \
                d.content_length, \
                dc.extracted_links_count, \
                EXTRACT(EPOCH FROM d.fetched_at)::BIGINT AS fetched_at, \
                EXTRACT(EPOCH FROM d.indexed_at)::BIGINT AS indexed_at, \
                dc.plain_text \
            FROM {schema}.documents d \
            INNER JOIN {schema}.document_contents dc ON dc.document_id = d.id \
            ORDER BY d.fetched_at DESC, d.id DESC \
            LIMIT $1",
            schema = self.schema
        );

        let rows = sqlx::query(&sql)
            .bind(i64::from(limit.max(1)))
            .fetch_all(&self.pool)
            .await?;

        rows.into_iter().map(row_to_list_item).collect()
    }

    pub async fn document_detail(
        &self,
        document_id: i64,
    ) -> Result<Option<ContentDetail>, AppError> {
        let sql = format!(
            "SELECT \
                d.id, \
                d.crawl_target_id, \
                d.canonical_url, \
                d.host, \
                d.path, \
                d.title, \
                d.content_type, \
                d.http_status, \
                d.language, \
                d.content_length, \
                dc.extracted_links_count, \
                EXTRACT(EPOCH FROM d.fetched_at)::BIGINT AS fetched_at, \
                EXTRACT(EPOCH FROM d.indexed_at)::BIGINT AS indexed_at, \
                dc.plain_text, \
                dc.raw_html \
            FROM {schema}.documents d \
            INNER JOIN {schema}.document_contents dc ON dc.document_id = d.id \
            WHERE d.id = $1",
            schema = self.schema
        );

        let row = sqlx::query(&sql)
            .bind(document_id)
            .fetch_optional(&self.pool)
            .await?;

        row.map(row_to_detail).transpose()
    }
}

fn row_to_list_item(row: sqlx::postgres::PgRow) -> Result<ContentListItem, AppError> {
    let plain_text: String = row.try_get("plain_text")?;
    let title: Option<String> = row.try_get("title")?;
    let summary = summarize_candidate(title.as_deref(), &plain_text);

    Ok(ContentListItem {
        id: row.try_get("id")?,
        crawl_target_id: row.try_get("crawl_target_id")?,
        canonical_url: row.try_get("canonical_url")?,
        host: row.try_get("host")?,
        path: row.try_get("path")?,
        title,
        summary,
        content_type: row.try_get("content_type")?,
        http_status: row.try_get("http_status")?,
        language: row.try_get("language")?,
        content_length: row.try_get("content_length")?,
        extracted_links_count: row.try_get("extracted_links_count")?,
        fetched_at: row.try_get("fetched_at")?,
        indexed_at: row.try_get("indexed_at")?,
    })
}

fn row_to_detail(row: sqlx::postgres::PgRow) -> Result<ContentDetail, AppError> {
    let plain_text: String = row.try_get("plain_text")?;
    let title: Option<String> = row.try_get("title")?;
    let summary = summarize_candidate(title.as_deref(), &plain_text);

    Ok(ContentDetail {
        id: row.try_get("id")?,
        crawl_target_id: row.try_get("crawl_target_id")?,
        canonical_url: row.try_get("canonical_url")?,
        host: row.try_get("host")?,
        path: row.try_get("path")?,
        title,
        summary,
        plain_text,
        raw_html: row.try_get("raw_html")?,
        content_type: row.try_get("content_type")?,
        http_status: row.try_get("http_status")?,
        language: row.try_get("language")?,
        content_length: row.try_get("content_length")?,
        extracted_links_count: row.try_get("extracted_links_count")?,
        fetched_at: row.try_get("fetched_at")?,
        indexed_at: row.try_get("indexed_at")?,
    })
}

fn summarize_candidate(title: Option<&str>, plain_text: &str) -> String {
    let body_summary = summarize_text(plain_text, 240);
    if !body_summary.is_empty() {
        return body_summary;
    }

    title.unwrap_or("Untitled document").to_string()
}
