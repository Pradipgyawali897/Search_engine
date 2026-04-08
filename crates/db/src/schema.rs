use crate::error::{DbError, DbResult};
use serde::{Deserialize, Serialize};
use url::Url;

pub const DEFAULT_POSTGRES_SCHEMA: &str = "search_engine";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlParts {
    pub canonical_url: String,
    pub scheme: String,
    pub host: String,
    pub path: String,
}

pub fn parse_canonical_url(raw_url: &str) -> DbResult<UrlParts> {
    let parsed = Url::parse(raw_url).map_err(|error| {
        DbError::Validation(format!("invalid canonical url '{raw_url}': {error}"))
    })?;
    let host = parsed.host_str().ok_or_else(|| {
        DbError::Validation(format!("canonical url '{raw_url}' must include a host"))
    })?;

    let path = if parsed.path().is_empty() {
        "/".to_string()
    } else {
        parsed.path().to_string()
    };

    Ok(UrlParts {
        canonical_url: parsed.to_string(),
        scheme: parsed.scheme().to_string(),
        host: host.to_string(),
        path,
    })
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LinkCategory {
    Visitable,
    Junk,
}

impl LinkCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visitable => "visitable",
            Self::Junk => "junk",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CrawlStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrawlTarget {
    pub id: Option<i64>,
    pub canonical_url: String,
    pub scheme: String,
    pub host: String,
    pub path: String,
    pub status: CrawlStatus,
    pub depth: i32,
    pub priority: i32,
    pub retry_count: i32,
}

impl CrawlTarget {
    pub fn new(canonical_url: impl Into<String>) -> DbResult<Self> {
        let canonical_url = canonical_url.into();
        let parts = parse_canonical_url(&canonical_url)?;

        Ok(Self {
            id: None,
            canonical_url: parts.canonical_url,
            scheme: parts.scheme,
            host: parts.host,
            path: parts.path,
            status: CrawlStatus::Pending,
            depth: 0,
            priority: 0,
            retry_count: 0,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Document {
    pub id: Option<i64>,
    pub crawl_target_id: Option<i64>,
    pub canonical_url: String,
    pub scheme: String,
    pub host: String,
    pub path: String,
    pub title: Option<String>,
    pub content_type: Option<String>,
    pub http_status: Option<i16>,
    pub etag: Option<String>,
    pub content_length: i64,
    pub checksum: Option<String>,
    pub language: Option<String>,
}

impl Document {
    pub fn new(canonical_url: impl Into<String>) -> DbResult<Self> {
        let canonical_url = canonical_url.into();
        let parts = parse_canonical_url(&canonical_url)?;

        Ok(Self {
            id: None,
            crawl_target_id: None,
            canonical_url: parts.canonical_url,
            scheme: parts.scheme,
            host: parts.host,
            path: parts.path,
            title: None,
            content_type: None,
            http_status: None,
            etag: None,
            content_length: 0,
            checksum: None,
            language: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DocumentContent {
    pub document_id: i64,
    pub raw_html: Option<String>,
    pub plain_text: String,
    pub extracted_links_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Term {
    pub id: Option<i64>,
    pub term: String,
    pub document_frequency: i64,
}

impl Term {
    pub fn new(term: impl Into<String>) -> DbResult<Self> {
        let normalized = term.into().trim().to_string();
        if normalized.is_empty() {
            return Err(DbError::Validation("term cannot be empty".to_string()));
        }

        Ok(Self {
            id: None,
            term: normalized,
            document_frequency: 0,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentTerm {
    pub document_id: i64,
    pub term_id: i64,
    pub term_frequency: i32,
}

impl DocumentTerm {
    pub fn new(document_id: i64, term_id: i64, term_frequency: i32) -> DbResult<Self> {
        if term_frequency <= 0 {
            return Err(DbError::Validation(
                "term frequency must be greater than zero".to_string(),
            ));
        }

        Ok(Self {
            document_id,
            term_id,
            term_frequency,
        })
    }
}

pub type Posting = DocumentTerm;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveredLink {
    pub url: String,
    pub category: LinkCategory,
    pub timestamp: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_document_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crawl_target_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_text: Option<String>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub depth: i32,
}

impl DiscoveredLink {
    pub fn new(url: impl Into<String>, category: LinkCategory, timestamp: i64) -> DbResult<Self> {
        let url = url.into();
        let canonical_url = parse_canonical_url(&url)?.canonical_url;

        Ok(Self {
            url: canonical_url,
            category,
            timestamp,
            source_document_id: None,
            crawl_target_id: None,
            anchor_text: None,
            depth: 0,
        })
    }
}

pub fn postgres_schema_statements(namespace: &str) -> DbResult<Vec<String>> {
    validate_schema_name(namespace)?;

    let crawl_status_type = format!("{namespace}.crawl_status");
    let link_category_type = format!("{namespace}.link_category");
    let crawl_targets = format!("{namespace}.crawl_targets");
    let documents = format!("{namespace}.documents");
    let document_contents = format!("{namespace}.document_contents");
    let terms = format!("{namespace}.terms");
    let document_terms = format!("{namespace}.document_terms");
    let discovered_links = format!("{namespace}.discovered_links");

    Ok(vec![
        format!("CREATE SCHEMA IF NOT EXISTS {namespace};"),
        format!(
            "DO $$ BEGIN \
                IF NOT EXISTS ( \
                    SELECT 1 \
                    FROM pg_type t \
                    JOIN pg_namespace n ON n.oid = t.typnamespace \
                    WHERE t.typname = 'crawl_status' AND n.nspname = '{namespace}' \
                ) THEN \
                    CREATE TYPE {crawl_status_type} AS ENUM ('pending', 'processing', 'completed', 'failed'); \
                END IF; \
            END $$;"
        ),
        format!(
            "DO $$ BEGIN \
                IF NOT EXISTS ( \
                    SELECT 1 \
                    FROM pg_type t \
                    JOIN pg_namespace n ON n.oid = t.typnamespace \
                    WHERE t.typname = 'link_category' AND n.nspname = '{namespace}' \
                ) THEN \
                    CREATE TYPE {link_category_type} AS ENUM ('visitable', 'junk'); \
                END IF; \
            END $$;"
        ),
        format!(
            "CREATE TABLE IF NOT EXISTS {crawl_targets} ( \
                id BIGSERIAL PRIMARY KEY, \
                canonical_url TEXT NOT NULL UNIQUE, \
                scheme TEXT NOT NULL, \
                host TEXT NOT NULL, \
                path TEXT NOT NULL DEFAULT '/', \
                status {crawl_status_type} NOT NULL DEFAULT 'pending', \
                depth INTEGER NOT NULL DEFAULT 0 CHECK (depth >= 0), \
                priority INTEGER NOT NULL DEFAULT 0, \
                retry_count INTEGER NOT NULL DEFAULT 0 CHECK (retry_count >= 0), \
                discovered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), \
                next_crawl_at TIMESTAMPTZ, \
                last_crawled_at TIMESTAMPTZ, \
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), \
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW() \
            );"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS crawl_targets_host_idx \
            ON {crawl_targets} (host);"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS crawl_targets_status_next_crawl_idx \
            ON {crawl_targets} (status, next_crawl_at);"
        ),
        format!(
            "CREATE TABLE IF NOT EXISTS {documents} ( \
                id BIGSERIAL PRIMARY KEY, \
                crawl_target_id BIGINT UNIQUE REFERENCES {crawl_targets}(id) ON DELETE CASCADE, \
                canonical_url TEXT NOT NULL UNIQUE, \
                scheme TEXT NOT NULL, \
                host TEXT NOT NULL, \
                path TEXT NOT NULL DEFAULT '/', \
                title TEXT, \
                content_type TEXT, \
                http_status SMALLINT, \
                etag TEXT, \
                content_length BIGINT NOT NULL DEFAULT 0 CHECK (content_length >= 0), \
                checksum TEXT, \
                language TEXT, \
                fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), \
                indexed_at TIMESTAMPTZ, \
                last_modified_at TIMESTAMPTZ, \
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), \
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW() \
            );"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS documents_host_idx \
            ON {documents} (host);"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS documents_fetched_at_idx \
            ON {documents} (fetched_at DESC);"
        ),
        format!(
            "CREATE TABLE IF NOT EXISTS {document_contents} ( \
                document_id BIGINT PRIMARY KEY REFERENCES {documents}(id) ON DELETE CASCADE, \
                raw_html TEXT, \
                plain_text TEXT NOT NULL DEFAULT '', \
                extracted_links_count INTEGER NOT NULL DEFAULT 0 CHECK (extracted_links_count >= 0), \
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), \
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW() \
            );"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS document_contents_search_idx \
            ON {document_contents} USING GIN (to_tsvector('simple', plain_text));"
        ),
        format!(
            "CREATE TABLE IF NOT EXISTS {terms} ( \
                id BIGSERIAL PRIMARY KEY, \
                term TEXT NOT NULL UNIQUE, \
                document_frequency BIGINT NOT NULL DEFAULT 0 CHECK (document_frequency >= 0), \
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), \
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW() \
            );"
        ),
        format!(
            "CREATE TABLE IF NOT EXISTS {document_terms} ( \
                document_id BIGINT NOT NULL REFERENCES {documents}(id) ON DELETE CASCADE, \
                term_id BIGINT NOT NULL REFERENCES {terms}(id) ON DELETE CASCADE, \
                term_frequency INTEGER NOT NULL CHECK (term_frequency > 0), \
                PRIMARY KEY (document_id, term_id) \
            );"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS document_terms_term_lookup_idx \
            ON {document_terms} (term_id, term_frequency DESC);"
        ),
        format!(
            "CREATE TABLE IF NOT EXISTS {discovered_links} ( \
                id BIGSERIAL PRIMARY KEY, \
                source_document_id BIGINT REFERENCES {documents}(id) ON DELETE SET NULL, \
                crawl_target_id BIGINT REFERENCES {crawl_targets}(id) ON DELETE SET NULL, \
                url TEXT NOT NULL, \
                category {link_category_type} NOT NULL, \
                anchor_text TEXT, \
                depth INTEGER NOT NULL DEFAULT 0 CHECK (depth >= 0), \
                discovered_at TIMESTAMPTZ NOT NULL DEFAULT NOW() \
            );"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS discovered_links_category_idx \
            ON {discovered_links} (category, discovered_at DESC);"
        ),
        format!(
            "CREATE INDEX IF NOT EXISTS discovered_links_url_idx \
            ON {discovered_links} (url);"
        ),
    ])
}

fn validate_schema_name(namespace: &str) -> DbResult<()> {
    let mut chars = namespace.chars();
    let Some(first) = chars.next() else {
        return Err(DbError::Validation(
            "postgres schema name cannot be empty".to_string(),
        ));
    };

    if !(first.is_ascii_alphabetic() || first == '_') {
        return Err(DbError::Validation(format!(
            "postgres schema name '{namespace}' must start with a letter or underscore"
        )));
    }

    if !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
        return Err(DbError::Validation(format!(
            "postgres schema name '{namespace}' may only contain letters, numbers, and underscores"
        )));
    }

    Ok(())
}

fn is_zero(value: &i32) -> bool {
    *value == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovered_link_serializes_like_existing_json_output() {
        let link = DiscoveredLink::new("https://example.com/page", LinkCategory::Junk, 42).unwrap();
        let json = serde_json::to_string(&link).unwrap();

        assert_eq!(
            json,
            r#"{"url":"https://example.com/page","category":"junk","timestamp":42}"#
        );
    }

    #[test]
    fn postgres_schema_requires_safe_namespace() {
        let error = postgres_schema_statements("search-engine").unwrap_err();
        assert!(
            error
                .to_string()
                .contains("may only contain letters, numbers, and underscores")
        );
    }

    #[test]
    fn postgres_schema_contains_core_tables() {
        let statements = postgres_schema_statements(DEFAULT_POSTGRES_SCHEMA).unwrap();
        let sql = statements.join("\n");

        assert!(sql.contains("CREATE TABLE IF NOT EXISTS search_engine.crawl_targets"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS search_engine.documents"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS search_engine.document_terms"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS search_engine.discovered_links"));
    }
}
