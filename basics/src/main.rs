#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

fn main() {
    println!("Hello, world!");
    let a = divide(15, 5);

    let b = divide(5, 0);

    match a {
        Res::Thing(v) => println!("a = {:?}", v),
        _ => {}, 
    }

    if let Res::Thing(v) = a {
        println!("val is {}", v);
    }

    match b {
        Res::Thing(v) => println!("b = {}", v),
        Res::Error(s) => println!("Error : {}", s),
    }
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Cannot divide by 0".to_string());
    }
    Res::Thing(a / b)
}

