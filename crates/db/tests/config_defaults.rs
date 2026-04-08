use db::{PostgresConfig, connect};

#[test]
fn postgres_config_uses_expected_defaults() {
    let config = PostgresConfig::new("postgres://localhost/pernox");

    assert_eq!(config.schema, "search_engine");
    assert_eq!(config.max_connections, 10);
    assert_eq!(config.min_connections, 1);
}

#[tokio::test]
async fn connect_rejects_empty_database_url() {
    let config = PostgresConfig::new("");
    let error = connect(&config).await.unwrap_err();
    assert!(
        error
            .to_string()
            .contains("postgres connection string cannot be empty")
    );
}
