use indexer::parse_html_document;

#[test]
fn verify_parsed_html_content() {
    let content = parse_html_document(
        r#"
        <html>
            <head><title>Pernox Docs</title></head>
            <body>
                <h1>Search Engine</h1>
                <p>Database-backed indexing pipeline.</p>
                <a href="/docs">Docs</a>
            </body>
        </html>
        "#,
        "http://example.test",
    );
    assert!(content.text.contains("Pernox Docs"));
    assert!(content.text.contains("Database-backed indexing pipeline."));
    assert_eq!(content.links, vec!["http://example.test/docs"]);
}
