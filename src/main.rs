fn main() {
    intro_to_u();
    string_handler();
}

fn intro_to_u() {
    let full_name = string_formatting(
        convert_to_string_v1("Akinshola"),
        convert_to_string_v2("Akinniyi"),
    );
    let (addition, substraction, multiplication, division) =
        arithmetic_signed_ops(2147483647, 2147483647);
}

fn string_formatting(first_name: String, last_name: String) -> String {
    let full_name = format!("{} {}", first_name, last_name);
    return full_name;
}

fn convert_to_string_v1(x: &str) -> String {
    x.to_string()
}

fn convert_to_string_v2(x: &str) -> String {
    String::from(x)
}

fn string_handler() {
    intro_to_ownable_string();
}

fn intro_to_str_slice<'a>() -> &'a str {
    let name: &str = "Sylvia";
    name
}

fn intro_to_ownable_string() {
    let mut name: String = String::from("Wisdom");
    name.push_str(" John");
}

fn integer_type_casting(first_int: u32, second_int: u8) -> (u8, u32) {
    (
        first_int.try_into().unwrap(),
        second_int.try_into().unwrap(),
    )
}

fn string_conversion_ops<'a>(string_literal: &str, main_string: &'a String) -> (String, &'a str) {
    (string_literal.to_string(), main_string.as_str())
}

fn arithmetic_signed_ops(first: i32, second: i32) -> (i32, i32, i32, i32) {
    let addition: i32 = first + second;
    let substraction: i32 = first - second;
    let multiplication: i32 = first * second;
    let division: i32 = first / second;
    return (addition, substraction, multiplication, division);
}

fn arithmetic_unsigned_ops(first: u32, second: u32) -> (u32, u32, u32, u32) {
    let addition: u32 = first + second;
    let substraction: u32 = first - second;
    let multiplication: u32 = first * second;
    let division: u32 = first / second;
    return (addition, substraction, multiplication, division);
}

fn arithmetic_floatingpoint_ops(first: f64, second: f64) -> (f64, f64, f64, f64) {
    let addition = first + second;
    let substraction = first - second;
    let multiplication = first * second;
    let divide = first / second;
    (addition, substraction, multiplication, divide)
}
