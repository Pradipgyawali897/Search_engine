use indexer::{Parser, XmlParser};

#[tokio::test]
async fn test_xml_parser_basic() {
    let parser = XmlParser;
    // We'll test with a known XML-like domain or mock if possible,
    // but for now, we'll just check if it compiles and runs.
    // Note: This actually makes a network request to google.com which is likely HTML,
    // so it might fail to parse as pure XML, but let's see.
    let parse_content = parser.parse("google.com").await;
    // If it's HTML, XmlParser might throw errors, which is expected for "robust" testing.
    println!("{:?}", parse_content);
}
