use std::collections::HashMap;

pub fn main() {
    let mut h= HashMap::new();

    h.insert("Math", 100);
    h.insert("Physics", 88);
    h.insert("Biology", 90);

    println!("{:?}", h);

    // Accessing values in HashMaps

    let first = h.get("");
    println!("First kv pair: {:?}", first);

    match first {
        Some(val) => println!("The value is: {}", val),
        None => println!("The value is out of range")
    }

    // Iterating over HashMaps

    for (subject, score) in h {
        println!("Key: {}, value: {}", subject, score)
    }
}