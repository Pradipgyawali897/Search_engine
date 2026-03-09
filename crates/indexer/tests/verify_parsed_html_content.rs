use indexer::{HtmlParser, Parser};

#[tokio::test]
async fn verify_parsed_html_content() {
    let parser = HtmlParser;
    let parse_content = parser.parse("google.com").await;
    assert!(parse_content.is_ok());
    let content = parse_content.unwrap();
    assert!(!content.is_empty());
}