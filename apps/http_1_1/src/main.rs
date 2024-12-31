use std::option::Option;
use http::Method; //can also use as http::method::Method, since we have added in http://mod.rs we will use shortcut everywhere
use server::Server;
use http::Request; //can also use as http::request::Request


mod server;
mod http;


fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    let string = String::from("127.0.0.1:8080");
    let server = Server::new(&string);
    server.run();
}







