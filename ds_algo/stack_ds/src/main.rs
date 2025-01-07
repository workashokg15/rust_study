
fn main() {
    println!("Hello, world!");
    let val5:i32 = factorial(5);
    let val7:i32 = factorial(7);
    let val9:i32 = factorial(9);

    println!("factorial(5) is : {}", val5);
    println!("factorial(7) is : {}", val7);
    println!("factorial(9) is : {}", val9);

    let val: i32 = factorial_dyn(5, 1);
    println!("factorial_dyn(5) is : {}", val);
    let val:i32 = factorial_dyn(7, 1);
    println!("factorial_dyn(7) is : {}", val);
    let val:i32 = factorial_dyn(9, 1);
    println!("factorial_dyn(9) is : {}", val);
}

fn factorial(n : i32) -> i32 {
    if n <=1 {
        return 1
    }
    return n * factorial(n - 1)
}

//Using tail recursion, it updates the return value in place
//so the stack does not grow.
fn factorial_dyn(n:i32, r:i32) -> i32 {
    if n <= 1 {
        return r
    }
    return factorial_dyn(n-1, n*r)
}