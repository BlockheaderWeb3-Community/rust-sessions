pub fn char_string() {
    let make_string: String = make_string("martin", "19");
    println!("my name is {}", make_string);
}

pub fn make_string(x: &str, y: &str) -> String {
    format!("{x} and i scored {y}")
}

pub fn intro_to_ownable_string() {
    let mut fname: String = String::from("martin");
    println!("first name: {} {}", fname, fname.len());

    fname.push_str("vibes");
    println!("first name_______: {} {}", fname, fname.len());
}