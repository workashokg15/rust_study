pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    pub fn run(&self) {
        println!("Server is running on {}", self.addr);
    }
}
