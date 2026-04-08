use db::queries::{contents, crawl_targets, documents, links, terms};

#[test]
fn query_builders_use_schema_scoped_tables() {
    let schema = "search_engine";

    let crawl_target_sql = crawl_targets::upsert_crawl_target_sql(schema).unwrap();
    let document_sql = documents::upsert_document_sql(schema).unwrap();
    let content_sql = contents::upsert_document_content_sql(schema).unwrap();
    let term_sql = terms::upsert_term_sql(schema).unwrap();
    let document_term_sql = terms::insert_document_term_sql(schema).unwrap();
    let link_sql = links::insert_discovered_link_sql(schema).unwrap();

    assert!(crawl_target_sql.contains("search_engine.crawl_targets"));
    assert!(crawl_target_sql.contains("search_engine.crawl_status"));
    assert!(document_sql.contains("search_engine.documents"));
    assert!(content_sql.contains("search_engine.document_contents"));
    assert!(term_sql.contains("search_engine.terms"));
    assert!(document_term_sql.contains("search_engine.document_terms"));
    assert!(link_sql.contains("search_engine.discovered_links"));
    assert!(link_sql.contains("search_engine.link_category"));
}
