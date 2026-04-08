use ab_web::storage::ContentStore;

#[tokio::test]
async fn content_store_new_accepts_valid_schema() {
    let store = ContentStore::new("postgresql://localhost/test", "search_engine");
    assert!(store.is_ok());
}

#[test]
fn content_store_new_rejects_invalid_schema() {
    let store = ContentStore::new("postgresql://localhost/test", "bad-schema");
    assert!(store.is_err());
}

#[test]
fn content_store_new_rejects_empty_database_url() {
    let store = ContentStore::new("   ", "search_engine");
    assert!(store.is_err());
}
