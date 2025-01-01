use std::option::Option;
use server::Server;

mod server;
fn main() {
    let server_address = "127.0.0.1:8080";
    let server = Server::new(&server_address);
    server.run();
}


