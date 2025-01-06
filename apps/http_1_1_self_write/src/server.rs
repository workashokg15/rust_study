use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    } 
    pub fn run(&self){
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Server is Listening on {}", self.addr)
    }
}
