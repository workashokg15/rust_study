#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {name, age}    
    }

    //arguements can be type self, mut self, &self, &mut self
    pub fn greet(&self)->String {
        format!("name : {}, age : {}", self.name, self.age)
    }

    pub fn dropme(self) {}
}


pub fn get_age(s:&Person) -> u32 {
    &s.age
}

fn main() {
    let p = Person::new("ashok".to_string(), 46);
    
    let s = p.greet();
    let s1 = p.greet();
    println!("s is {}", s);
    println!("s1 is {}", s1);
}


