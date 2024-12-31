fn main() {
    println!("Hello, world!");
    Server::new("127.0.0.1", 8080).start();
}

struct Server {
    host: String,
    port: u16,
}

impl Server {
    fn new(host: &str, port: u16) -> Server {
        Server {
            host: host.to_string(),
            port,
        }
    }

    fn start(&self) {
        println!("Server started at {}:{}", self.host, self.port);
    }
}
