use tiny_http::{Response, Server};

pub fn start_server(port: Option<&str>) {
    let port = port.unwrap_or("127.0.0.1:6969");
    let server = Server::http(port)
        .map_err(|err| {
            eprintln!("Error starting the server at {port}: {err}");
        })
        .unwrap();

    println!("Server running at http://{}", port);

    for request in server.incoming_requests() {
        println!(
            "Received request: {} {}",
            request.method(),
            request.url()
        );
        let response = Response::from_string("Search Engine API").with_status_code(200);
        let _ = request.respond(response);
    }
}
