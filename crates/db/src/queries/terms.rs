use crate::error::DbResult;
use crate::schema::validate_schema_name;

pub fn upsert_term_sql(schema: &str) -> DbResult<String> {
    validate_schema_name(schema)?;

    Ok(format!(
        "INSERT INTO {schema}.terms (term, updated_at) \
        VALUES ($1, NOW()) \
        ON CONFLICT (term) DO UPDATE SET updated_at = NOW() \
        RETURNING id;"
    ))
}

pub fn delete_document_terms_sql(schema: &str) -> DbResult<String> {
    validate_schema_name(schema)?;
    Ok(format!(
        "DELETE FROM {schema}.document_terms WHERE document_id = $1;"
    ))
}

pub fn insert_document_term_sql(schema: &str) -> DbResult<String> {
    validate_schema_name(schema)?;
    Ok(format!(
        "INSERT INTO {schema}.document_terms (document_id, term_id, term_frequency) \
        VALUES ($1, $2, $3) \
        ON CONFLICT (document_id, term_id) DO UPDATE SET term_frequency = EXCLUDED.term_frequency;"
    ))
}
