use indexer::parse_xml_document;

#[test]
fn test_xml_parser_basic() {
    let content = parse_xml_document(
        r#"<?xml version="1.0" encoding="UTF-8"?><root><title>Pernox</title><body>Structured XML response</body></root>"#,
    )
    .unwrap();
    assert!(content.text.contains("Pernox"));
    assert!(content.text.contains("Structured XML response"));
}
