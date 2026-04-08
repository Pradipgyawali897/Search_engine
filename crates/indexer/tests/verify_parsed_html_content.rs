use indexer::{HtmlParser, Parser};
use tiny_http::{Header, Response, Server};

#[tokio::test]
async fn verify_parsed_html_content() {
    let server = Server::http("127.0.0.1:0").unwrap();
    let address = format!("http://{}", server.server_addr());

    let handle = std::thread::spawn(move || {
        let request = server.recv().unwrap();
        let response = Response::from_string(
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
        )
        .with_header(Header::from_bytes("Content-Type", "text/html; charset=utf-8").unwrap());

        request.respond(response).unwrap();
    });

    let parser = HtmlParser;
    let parse_content = parser.parse(&address).await;
    assert!(parse_content.is_ok());
    let content = parse_content.unwrap();
    assert!(content.text.contains("Pernox Docs"));
    assert!(content.text.contains("Database-backed indexing pipeline."));
    assert_eq!(content.links, vec![format!("{}/docs", address)]);

    handle.join().unwrap();
}
