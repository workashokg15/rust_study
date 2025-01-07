struct Stepper {
    curr: i32,
    step: i32, 
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self)->Option<i32> {
        if self.curr > self.max {
            return None;
        }
        let temp = self.curr;
        self.curr += self.step;
        Some(temp)
    }
}

fn main() {
    let mut n = 0;
    loop {
        n += 1;
        if n > 10 {
            break;
        }
        println!("loop Hello, world!");
    }

    //While
    n = 0;
    while n < 10 {
        println!("While Hello, world!");
        n += 1;
    }

    //for loop
    for i in 0..10 { //creates a range object or stepper object.
        println!("for hello world loop number {}", i)
    }

    //Using stepper
    let mut st = Stepper{curr: 2, step: 4, max: 25};

    loop {
        match st.next() {
            Some(v) => println!("curr is : {}" , v),
            None => break,
        }
    }
    //while use using stepper

    let mut st = Stepper{curr: 3, step: 2, max: 16};
    while let Some(n) = st.next(){
        println!("while loop stepper {}", n);    
    }

    let it = Stepper{curr: 5, step: 1, max: 22};
    for n in it {
        println!("for loop n : {}", n);
    }
}
