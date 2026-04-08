use crate::error::DbResult;
use crate::schema::validate_schema_name;

pub fn upsert_crawl_target_sql(schema: &str) -> DbResult<String> {
    validate_schema_name(schema)?;

    Ok(format!(
        "INSERT INTO {schema}.crawl_targets \
        (canonical_url, scheme, host, path, status, depth, priority, retry_count) \
        VALUES ($1, $2, $3, $4, $5::{schema}.crawl_status, $6, $7, $8) \
        ON CONFLICT (canonical_url) DO UPDATE SET \
            scheme = EXCLUDED.scheme, \
            host = EXCLUDED.host, \
            path = EXCLUDED.path, \
            status = EXCLUDED.status, \
            depth = EXCLUDED.depth, \
            priority = EXCLUDED.priority, \
            retry_count = EXCLUDED.retry_count, \
            updated_at = NOW() \
        RETURNING id;"
    ))
}
