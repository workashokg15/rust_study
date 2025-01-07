#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

//AddAssign trait provides += feature, T is the type of trait
impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut ll = LinkedList{data:32, next:Some(Box::new(LinkedList{data:34, next:None}))};
    
    if let Some(ref mut v) = ll.next {
        v.add_up(10);
    }

    //let mut v:Vec<String> = Vec::new();
    let mut v = Vec::with_capacity(100);
    v.push("hello".to_string());
    v.push("my".to_string());
    v.push("dear".to_string());

    println!("Linked list is : {:?}", ll);
    
    println!("capacity of v : {}, length of v is : {}", v.capacity(), v.len());
    println!("v is : {:?}", v);

    for i in 0..105 {
        v.push(i.to_string());
    }
    println!("check new capacity and values");
    println!("capacity of v : {}, length of v is : {}", v.capacity(), v.len());
    println!("v is : {:?}", v);

}
