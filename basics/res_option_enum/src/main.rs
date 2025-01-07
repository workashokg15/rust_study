use std::result::Result;

pub enum Res<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let a = divide (5, 2);
    let b = divide1 (5, 0);

    match a {
        Res::Ok(v) => println!("Res is : {}", v),
        _ => {},
    }

    match b {
        Result::Ok(v) => println!("Res is : {}", v),
        Result::Err(s) => println!("{}", s),
    }
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Err("Cannot divide by zero".to_string());
    }
    Res::Ok(a / b)
}

//Use inbuild Result
fn divide1(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Result::Err("Cannot divide by zero".to_string());
    }
    Result::Ok(a / b)
}