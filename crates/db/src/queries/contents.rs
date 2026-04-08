use crate::error::DbResult;
use crate::schema::validate_schema_name;

pub fn upsert_document_content_sql(schema: &str) -> DbResult<String> {
    validate_schema_name(schema)?;

    Ok(format!(
        "INSERT INTO {schema}.document_contents \
        (document_id, raw_html, plain_text, extracted_links_count, updated_at) \
        VALUES ($1, $2, $3, $4, NOW()) \
        ON CONFLICT (document_id) DO UPDATE SET \
            raw_html = EXCLUDED.raw_html, \
            plain_text = EXCLUDED.plain_text, \
            extracted_links_count = EXCLUDED.extracted_links_count, \
            updated_at = NOW();"
    ))
}
