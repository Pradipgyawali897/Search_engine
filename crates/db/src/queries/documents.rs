use crate::error::DbResult;
use crate::schema::validate_schema_name;

pub fn upsert_document_sql(schema: &str) -> DbResult<String> {
    validate_schema_name(schema)?;

    Ok(format!(
        "INSERT INTO {schema}.documents \
        (crawl_target_id, canonical_url, scheme, host, path, title, content_type, http_status, etag, content_length, checksum, language, fetched_at, indexed_at, updated_at) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW(), NOW(), NOW()) \
        ON CONFLICT (canonical_url) DO UPDATE SET \
            crawl_target_id = EXCLUDED.crawl_target_id, \
            scheme = EXCLUDED.scheme, \
            host = EXCLUDED.host, \
            path = EXCLUDED.path, \
            title = EXCLUDED.title, \
            content_type = EXCLUDED.content_type, \
            http_status = EXCLUDED.http_status, \
            etag = EXCLUDED.etag, \
            content_length = EXCLUDED.content_length, \
            checksum = EXCLUDED.checksum, \
            language = EXCLUDED.language, \
            fetched_at = NOW(), \
            indexed_at = NOW(), \
            updated_at = NOW() \
        RETURNING id;"
    ))
}
