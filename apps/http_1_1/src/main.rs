use core::slice;

fn main() {
    // let string = String::from("127.0.0.1:8080");
    // let string1: String = String::from("😻😻");
    // let c: char = string1[0..4].chars().next().unwrap();
    // let start_port = string.find(':').unwrap();
    // let string_slice= &string[start_port..];
    // let string_borrow: &str = &string;
    // let string_literal = "1234";
    // let bing = "bing".to_string();

    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(string_borrow);
    // dbg!(string_literal);
    // dbg!(bing);
    // dbg!(string1);
    // dbg!(c);

    let server = Server::new(string_slice);
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    fn run(&self) {
        println!("Server is running on {}", self.addr);
    }
}