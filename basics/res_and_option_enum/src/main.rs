use std::result::Result;

#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

pub enum Option<T> {
    Some(T),
    None,
}

fn main() {
    println!("Hello, world!");
    let a = divide(15, 5);

    let b = divide(5, 0);

    match a {
        Result::Ok(v) => println!("a = {}", v),
        _ => {}, 
    }

    if let Result::Ok(v) = a {
        println!("val is {}", v);
    }

    match b {
        Result::Ok(v) => println!("b = {}", v),
        Result::Err(s) => println!("Error : {}", s),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Result::Err("Cannot divide by 0".to_string());
    }
    Result::Ok(a / b)
}

