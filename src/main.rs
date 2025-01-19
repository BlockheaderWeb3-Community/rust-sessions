fn main() {
    // intro_to_u();
    string_handler();
    println!(".......................................................");
    type_conversion();
    println!(".......................................................");
    signed_integers();
    println!(".......................................................");
    string_conversion();
}


// function to encapsulate all integers
fn intro_to_u(){
    let sum_result: u8 = sum(5, 10);
    let mult_result: u64 = multiply(5, 10);
    let divide: f64 = divide(20.0, 10.2);
    let subtr: isize = substract(20, 10);
    let check: bool = check_func(5, 10);
    println!("Sum: {}", sum_result);
    println!("Multiplication: {}", mult_result);
    println!("Division: {}", divide);
    println!("Substraction: {}", subtr);    
    println!("Check: {}", check);

    let sum_result: f64 = sumfp(5.0, 10.0);
    let mult_result: f64 = multiply_fp(5.0, 10.0);
    let divide: f64 = divide_fp(20.0, 10.2);
    let subtr: f64 = substract_fp(20.0, 10.0);
    println!("Sum: {}", sum_result);
    println!("Multiplication: {}", mult_result);
    println!("Division: {}", divide);
    println!("Substraction: {}", subtr);

    let full_name = string_formatting(convert_to_string_v1("Akinshola"), convert_to_string_v2("Akinniyi"));
    println!("Full Name: {}", full_name);
}

fn sum(x: u8, y: u8) -> u8 {
    x + y
}
fn multiply(x: u64, y: u64) -> u64 {
    x * y
}
fn divide(x: f64, y: f64) -> f64 {
    let res: f64 = x / y;
    return res
}
fn substract(x: isize, y: isize) -> isize {
    x - y
}

fn sumfp(x: f64, y: f64) -> f64 {
    x + y
}

fn multiply_fp(x: f64, y: f64) -> f64 {
    x * y
}

fn divide_fp(x: f64, y: f64) -> f64 {
    x / y
}

fn substract_fp(x: f64, y: f64) -> f64 {
    x - y
}

fn check_func(num1: u8, num2: u8) -> bool {
    let sum_of_two_nums = sum(num1, num2);
    if sum_of_two_nums % 2 == 0 {
        println!("The sum of {} and {} is even", num1, num2);
        return true;
    } else {
        println!("The sum of {} and {} is odd", num1, num2);
        return false;
    }
}

// fn string_formatting(first_name: &str, last_name: &str) -> String {
//     let full_name = format!("{} {}", first_name, last_name);
//     return full_name;
// }
fn string_formatting(first_name: String, last_name: String) -> String {
    let full_name = format!("{} {}", first_name, last_name);
    return full_name;
}

// util fn version 1 to convert &str to String 
fn convert_to_string_v1(x: &str) -> String {
    x.to_string()
}

// util fn version 2 to convert &str to String 
fn convert_to_string_v2(x: &str) -> String {
   String::from(x)
}


// handle all string-related functions
fn string_handler() {
    // intro_to_str_slice();
    intro_to_ownable_string();
}

// intro string slice
// for fixed-sized strings
fn intro_to_str_slice() {
    let name: &str = "Sylvia";
    println!("my name is {}", name)
}

fn intro_to_ownable_string() {
    let mut name: String = String::from("Wisdom");
    println!("first name: here: {}", name);
    name.push_str(" John");
    println!("final name: here: {}", name);
    println!("ptr = address in heap memory: {:?}", name.as_ptr());
}

fn type_conversion(){
    let sum_casting: u32 = sum_cast(50, 50);
    println!("The sum of the two values is {}", sum_casting);
    let product_casting: u8 = multiply_cast(10, 15);
    println!("The product of the two values is {}", product_casting);
}

// Type casting 1
fn sum_cast(x: u8, y: u8) -> u32 {
    let result = x + y;
    result as u32
}

// Type casting 2
fn multiply_cast(p: u32, q: u32) -> u8 {
    let answer = p * q;
    answer as u8    
}

fn signed_integers(){
let signed_sum: i8 = sum_signed(19, 19);
let signed_subtraction: i8 = substract_signed(100, 50);
let signed_multiplication: i8 = multiply_signed(10, 12);
let signed_division: f32 = divide_signed(100, 18);
println!("The sum of the two values is {}", signed_sum);
println!("The difference between the two values is {}", signed_subtraction);
println!("The product of the two values is {}", signed_multiplication);
println!("The division of the two values is {}", signed_division);

}

fn sum_signed(a: i8, b: i8)-> i8{
    a + b
}
fn substract_signed(a: i32, b:i32)-> i8{
    return (a-b)as i8;
}
fn multiply_signed(a: i8, b: i8)-> i8{
    a * b
}
fn divide_signed(a: i64, b: i64)-> f32{
    return a as f32 / b as f32;
}

fn string_to_str(input: &String) -> &str{
    input.as_str()
}

fn string_conversion(){
    let string_input = String::from("My name is Wisdom Enyinnaya");
    let referenced_string = string_to_str(&string_input);

    println!("String: {}", string_input);
    println!("&str: {}", referenced_string);
}


