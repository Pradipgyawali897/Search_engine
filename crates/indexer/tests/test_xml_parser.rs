use indexer::{Parser, XmlParser};
use tiny_http::{Header, Response, Server};

#[tokio::test]
async fn test_xml_parser_basic() {
    let server = Server::http("127.0.0.1:0").unwrap();
    let address = format!("http://{}", server.server_addr());

    let handle = std::thread::spawn(move || {
        let request = server.recv().unwrap();
        let response = Response::from_string(
            r#"<?xml version="1.0" encoding="UTF-8"?><root><title>Pernox</title><body>Structured XML response</body></root>"#,
        )
        .with_header(
            Header::from_bytes("Content-Type", "application/xml; charset=utf-8").unwrap(),
        );

        request.respond(response).unwrap();
    });

    let parser = XmlParser;
    let parse_content = parser.parse(&address).await;
    assert!(parse_content.is_ok());

    let content = parse_content.unwrap();
    assert!(content.text.contains("Pernox"));
    assert!(content.text.contains("Structured XML response"));

    handle.join().unwrap();
}
