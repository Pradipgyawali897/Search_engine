use crate::error::DbResult;
use crate::schema::validate_schema_name;

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
