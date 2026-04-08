use db::schema::{DEFAULT_POSTGRES_SCHEMA, postgres_schema_statements};

#[test]
fn postgres_schema_contains_core_tables() {
    let statements = postgres_schema_statements(DEFAULT_POSTGRES_SCHEMA).unwrap();
    let sql = statements.join("\n");

    assert!(sql.contains("CREATE TABLE IF NOT EXISTS search_engine.crawl_targets"));
    assert!(sql.contains("CREATE TABLE IF NOT EXISTS search_engine.documents"));
    assert!(sql.contains("CREATE TABLE IF NOT EXISTS search_engine.document_terms"));
    assert!(sql.contains("CREATE TABLE IF NOT EXISTS search_engine.discovered_links"));
}

#[test]
fn postgres_schema_rejects_invalid_namespace() {
    let error = postgres_schema_statements("search-engine").unwrap_err();
    assert!(
        error
            .to_string()
            .contains("may only contain letters, numbers, and underscores")
    );
}
