use crate::error::DbResult;
use crate::schema::validate_schema_name;

pub fn upsert_crawl_target_sql(schema: &str) -> DbResult<String> {
    validate_schema_name(schema)?;

    Ok(format!(
        "WITH inserted AS ( \
            INSERT INTO {schema}.crawl_targets \
            (canonical_url, scheme, host, path, status, depth, priority, retry_count) \
            VALUES ($1, $2, $3, $4, $5::{schema}.crawl_status, $6, $7, $8) \
            ON CONFLICT (canonical_url) DO NOTHING \
            RETURNING id \
        ) \
        SELECT id FROM inserted \
        UNION ALL \
        SELECT id FROM {schema}.crawl_targets WHERE canonical_url = $1 \
        LIMIT 1;"
    ))
}
