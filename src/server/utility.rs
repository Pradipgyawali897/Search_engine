use std::fs::File;
use tiny_http::Response;


pub struct Route{
    pub file:&str,
    pub method:tiny_http::Method,
    pub handler:fn(tiny_http::Request) -> tiny_http::Response,
    pub pat:&str,
}


pub fn serve_request(request: tiny_http::Request, file:&str) -> tiny_http::Response {
    let file=File::open(file).unwrap_or_else(|_| {
        Response::from_string("Not Found").with_status_code(404)
    });
    let response=Response::from_file(file).with_status_code(200);
    if(response.is_err()){
        Response::from_string("Not Found").with_status_code(404)
    }
}


pub fn handel_route(){

}