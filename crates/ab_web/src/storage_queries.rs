use crate::error::AppError;

pub fn validate_schema_name(namespace: &str) -> Result<(), AppError> {
    let mut chars = namespace.chars();
    let Some(first) = chars.next() else {
        return Err(AppError::config(
            "postgres schema name cannot be empty",
        ));
    };

    if !(first.is_ascii_alphabetic() || first == '_') {
        return Err(AppError::config(format!(
            "postgres schema name '{namespace}' must start with a letter or underscore"
        )));
    }

    if !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
        return Err(AppError::config(format!(
            "postgres schema name '{namespace}' may only contain letters, numbers, and underscores"
        )));
    }

    Ok(())
}

pub fn latest_documents_sql(schema: &str) -> Result<String, AppError> {
    qualify_schema(schema)?;
    Ok(format!(
        "SELECT \
            d.id, d.canonical_url, d.title, d.host, d.path, d.content_type, \
            d.content_length, COALESCE(c.extracted_links_count, 0) AS extracted_links_count, \
            EXTRACT(EPOCH FROM d.fetched_at)::BIGINT AS fetched_at_epoch, \
            CASE \
                WHEN d.indexed_at IS NULL THEN NULL \
                ELSE EXTRACT(EPOCH FROM d.indexed_at)::BIGINT \
            END AS indexed_at_epoch, \
            LEFT(COALESCE(c.plain_text, ''), 600) AS preview \
        FROM {schema}.documents d \
        LEFT JOIN {schema}.document_contents c ON c.document_id = d.id \
        ORDER BY COALESCE(d.indexed_at, d.fetched_at) DESC, d.id DESC \
        LIMIT $1"
    ))
}

pub fn document_detail_sql(schema: &str) -> Result<String, AppError> {
    qualify_schema(schema)?;
    Ok(format!(
        "SELECT \
            d.id, d.canonical_url, d.title, d.host, d.path, d.content_type, \
            d.content_length, COALESCE(c.extracted_links_count, 0) AS extracted_links_count, \
            EXTRACT(EPOCH FROM d.fetched_at)::BIGINT AS fetched_at_epoch, \
            CASE \
                WHEN d.indexed_at IS NULL THEN NULL \
                ELSE EXTRACT(EPOCH FROM d.indexed_at)::BIGINT \
            END AS indexed_at_epoch, \
            COALESCE(c.plain_text, '') AS plain_text, \
            c.raw_html \
        FROM {schema}.documents d \
        LEFT JOIN {schema}.document_contents c ON c.document_id = d.id \
        WHERE d.id = $1 \
        LIMIT 1"
    ))
}

pub fn latest_document_sql(schema: &str) -> Result<String, AppError> {
    qualify_schema(schema)?;
    Ok(format!(
        "SELECT \
            d.id, d.canonical_url, d.title, d.host, d.path, d.content_type, \
            d.content_length, COALESCE(c.extracted_links_count, 0) AS extracted_links_count, \
            EXTRACT(EPOCH FROM d.fetched_at)::BIGINT AS fetched_at_epoch, \
            CASE \
                WHEN d.indexed_at IS NULL THEN NULL \
                ELSE EXTRACT(EPOCH FROM d.indexed_at)::BIGINT \
            END AS indexed_at_epoch, \
            COALESCE(c.plain_text, '') AS plain_text, \
            c.raw_html \
        FROM {schema}.documents d \
        LEFT JOIN {schema}.document_contents c ON c.document_id = d.id \
        ORDER BY COALESCE(d.indexed_at, d.fetched_at) DESC, d.id DESC \
        LIMIT 1"
    ))
}

pub fn document_by_url_sql(schema: &str) -> Result<String, AppError> {
    qualify_schema(schema)?;
    Ok(format!(
        "SELECT \
            d.id, d.canonical_url, d.title, d.host, d.path, d.content_type, \
            d.content_length, COALESCE(c.extracted_links_count, 0) AS extracted_links_count, \
            EXTRACT(EPOCH FROM d.fetched_at)::BIGINT AS fetched_at_epoch, \
            CASE \
                WHEN d.indexed_at IS NULL THEN NULL \
                ELSE EXTRACT(EPOCH FROM d.indexed_at)::BIGINT \
            END AS indexed_at_epoch, \
            COALESCE(c.plain_text, '') AS plain_text, \
            c.raw_html \
        FROM {schema}.documents d \
        LEFT JOIN {schema}.document_contents c ON c.document_id = d.id \
        WHERE d.canonical_url = $1 \
        LIMIT 1"
    ))
}

fn qualify_schema(schema: &str) -> Result<(), AppError> {
    validate_schema_name(schema)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_sql_with_schema_name() {
        let list_sql = latest_documents_sql("search_engine").unwrap();
        let detail_sql = document_detail_sql("search_engine").unwrap();
        let latest_sql = latest_document_sql("search_engine").unwrap();
        let by_url_sql = document_by_url_sql("search_engine").unwrap();

        assert!(list_sql.contains("search_engine.documents"));
        assert!(detail_sql.contains("search_engine.document_contents"));
        assert!(latest_sql.contains("ORDER BY COALESCE(d.indexed_at, d.fetched_at) DESC"));
        assert!(by_url_sql.contains("WHERE d.canonical_url = $1"));
    }

    #[test]
    fn rejects_invalid_schema_names() {
        assert!(validate_schema_name("").is_err());
        assert!(validate_schema_name("1bad").is_err());
        assert!(validate_schema_name("bad-schema").is_err());
    }
}
