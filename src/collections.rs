use std::collections::HashMap;
// Given a Haystack(Vector of items), search for a needle inside.

// Haystack of numbers, search for 47
fn number_search(x: Vec<i32>, z: i32) {
    let num = x.iter().find(|y| *y == &z);
    match num {
        Some(a) => println!("{} exist", a),
        None => println!("number does not exist"),
    }
}
// Haystack of strings, search for Agent.
fn string_search(x: Vec<&str>, z: &str) {
    let num = x.iter().find(|y| *y == &z);
    match num {
        Some(a) => println!("{} exist", a),
        None => println!("string does not exist"),
    }
}

// Given a string of characters
// Convert it to an array of characters
// Print all the characters
// Convert the characters back to the given string.
fn string_to_char_to_string() {
    let name = String::from("Eden Hazard");
    let mut l = String::new();
    name.chars().into_iter().for_each(|x| {
        println!("{x}");
        l.push(x);
    });
    println!("{l}");
}

// Given a sentence, count the word occurences in the sentence.
fn word_occurance_count() {
    let mut scores = HashMap::new();
    let club = String::from("chelsea manchester barcelona arsenal chelsea");

    for word in club.split_whitespace() {
        let count = scores.entry(word).or_insert(0);
        *count += 1
    }
    println!("hash map {:#?}", scores);
}

// Given a set of values associated to a particular person,
// model an efficient structure that helps them store these values.
// Keep track of all the people you store values for
// and the values they are storing.
// Set of values => Vec, Array, Slices, Tuples...
// People => HashMap, Vec, HashSet...
fn user_provision_model(username: &str) {
    let mut provision = HashMap::new();
    provision.insert(
        "kate",
        vec![
            (1, "suger"),
            (2, "maggi"),
            (3, "rice"),
            (4, "beans"),
            (4, "4 cup of garri"),
        ],
    );
    provision.insert(
        "Eden",
        vec![
            (17, "milk"),
            (42, "mango"),
            (3, "rice"),
            (4, "plantain"),
            (3, "onion"),
        ],
    );

    //provision.entry(username).or_insert(vec![(0,"no data")]);
    let user = provision.get(username);
    match user {
        Some(data) => println!("user data {:#?}", data),
        _ => println!("user does not exist"),
    }
}

pub fn collections() {
    println!("-----------COLLECTIONS----------");
    string_to_char_to_string();
    word_occurance_count();
    user_provision_model("Eden");
    user_provision_model("Messi");

    let mut numbers = Vec::new();

    for i in 1..=100 {
        numbers.push(i);
    }

    let strings = vec!["kate", "bravoos", "agent", "india", "toronto", "needle"];
    number_search(numbers, 100);
    string_search(strings, "agent");
}
