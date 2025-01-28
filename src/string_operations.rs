// use crate::utils;

// Function to format two strings as a full name
pub fn string_formating_names(first_name: &str, last_name: &str) -> String {
    format!("{} {}", first_name, last_name) // Concatenate first and last names
}

// Function to convert a string slice to a `String` using `to_string`
pub fn convert_to_string_i1(x: &str) -> String {
    x.to_string()
}

// Function to convert a string slice to a `String` using `String::from`
pub fn convert_to_string_i2(x: &str) -> String {
    String::from(x)
}
