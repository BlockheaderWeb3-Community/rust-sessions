pub fn main() {
    let mut s = String::new();

    s.push_str("Hello World\n");
    s.push_str(", and Hi");

    // Pushing a char
    s.push('a');

    println!("{}", s);

    // Iterating over the chars in a String
    for c in s.chars() {
        println!("{}", c);
    }
}