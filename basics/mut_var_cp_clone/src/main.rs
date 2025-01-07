#[derive(Debug, Clone)]
struct Person {
    age: u32,
    name: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    x: u32,
    y: u32,
}

impl Person {
    // pub fn new(age: u32, name: String)->Self {
    //     Person(age, name)
    // }
}

impl Rect {
    pub fn new(x: u32, y:u32) -> Self {
        Rect{x, y}
    }
}

fn main() {
    let mut p = Person {
        age: 15, 
        name: "suresh".to_string(),
    };

    let p2 = p.clone();
    p.name.push_str(" lalit");

    println!("p2 is : {:?} and p is {:?}", p2, p);

    let mut r = Rect::new(20, 30);
    let r1 = r;
    r.x = 33;
    println!("r1 is : {:?} and r is : {:?}", r1, r);
}

