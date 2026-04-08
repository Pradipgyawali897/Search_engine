use crate::error::DbResult;
use crate::schema::validate_schema_name;

pub fn insert_discovered_link_sql(schema: &str) -> DbResult<String> {
    validate_schema_name(schema)?;

    Ok(format!(
        "INSERT INTO {schema}.discovered_links \
        (source_document_id, crawl_target_id, url, category, anchor_text, depth) \
        VALUES ($1, $2, $3, $4::{schema}.link_category, $5, $6);"
    ))
}
