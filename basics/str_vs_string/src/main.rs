fn main() {
    let mut s = " hello ".to_string();
    let p = s.trim();
    //Below statement will drop the immutability of s,
    //since p is going to heap
    let p = p.to_string();

    s.push_str("goodbye");

    println!("p = '{}'", p);

    let fstr = "help me find home";
    let ffstr = string_find_f(fstr);
    println!("ffstr = '{}'", ffstr);

    println!("{}",choose_str(0));
}

//add lifetime match
fn string_find_f<'a>(s: &'a str) -> &'a str {
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }
    s
}

fn choose_str(n: i32) -> &'static str {
    match n {
        0 => "hello",
        1 => "me",
        _ => "other",
    }
}