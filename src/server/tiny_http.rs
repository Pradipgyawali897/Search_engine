use tiny_http::{Response, Server};

pub fn start_server(port: Option<&str>) {
    let port = port.unwrap_or("127.0.0.0:6969");
    let server = Server::http(port)
        .map_err(|err| {
            eprintln!("Error caused on starting the server at {port} \n {err} ");
        })
        .unwrap();
    for request in server.incoming_requests() {
        println!(
            "Recived the request! method{:?} ,url{:?}",
            request.method(),
            request.url()
        );
        let response = Response::from_string("Not Found").with_status_code((404));
        request.respond(response).unwrap();
    }

    println!("Starting the server at the address {port}");
}
