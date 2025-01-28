pub fn string_formatting(first_name: String, last_name: String) -> String {
    let full_name = format!("{} {}", first_name, last_name);
    return full_name;
}

pub fn string_handler() {
    intro_to_ownable_string();
}

pub fn intro_to_str_slice<'a>() -> &'a str {
    let name: &str = "Sylvia";
    name
}

pub fn intro_to_ownable_string() {
    let mut name: String = String::from("Wisdom");
    name.push_str(" John");
}

pub fn string_conversion_ops<'a>(
    string_literal: &str,
    main_string: &'a String,
) -> (String, &'a str) {
    (string_literal.to_string(), main_string.as_str())
}
