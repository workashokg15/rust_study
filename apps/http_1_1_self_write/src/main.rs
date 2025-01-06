use std::option::Option;
use server::Server;
use http::Method;
use http::Request;

mod server;
mod http;
fn main() {
    let get: Method = Method::GET;
 
    let server_address = "127.0.0.1:8080";
    let server = Server::new(&server_address);
    server.run();
}




