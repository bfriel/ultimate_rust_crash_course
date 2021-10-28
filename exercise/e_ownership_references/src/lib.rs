pub fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    };
}

pub fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    };
}

pub fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a") 
}

pub fn add(x: &u32, y: &u32) -> u32 {
    let x = *x;
    let y = *y;
    x + y
}