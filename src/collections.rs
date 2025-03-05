// Given a Haystack(Vector of items), search for a needle inside.
// Haystack of numbers, search for 47
// Haystack of strings, search for Agent.

fn search_needle<T: PartialEq>(haystack: Vec<T>, needle: T) -> Option<usize> {
    haystack.iter().position(|&item| item == needle)
}

fn main() {
    // Haystack of numbers, search for 47
    let numbers: Vec<i32> = vec![10, 25, 47, 30, 15];
    match search_needle(numbers.clone(), 47) {
        Some(index) => println!("Found 47 at index {}", index),
        None => println!("47 not found in the numbers"),
    }

    // Haystack of strings, search for "Agent"
    let strings: Vec<String> = vec![
        "Smith".to_string(),
        "Agent".to_string(),
        "Neo".to_string(),
    ];
    match search_needle(strings.clone(), "Agent".to_string()) {
        Some(index) => println!("Found 'Agent' at index {}", index),
        None => println!("'Agent' not found in the strings"),
    }
}

// Given a string of characters
// Convert it to an array of characters
// Print all the characters
// Convert the characters back to the given string.

// Given a sentence, count the word occurences in the sentence.

// Given a set of values associated to a particular person,
// model an efficient structure that helps them store these values.
// Keep track of all the people you store values for
// and the values they are storing.
// Set of values => Vec, Array, Slices, Tuples...
// People => HashMap, Vec, HashSet...