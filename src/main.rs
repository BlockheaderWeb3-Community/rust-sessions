fn main() {
    intro_to_u();
    string_handler();
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
    let sum_result: u8 = sum(5, 10);
    println!("the sum result is: {}", sum_result);
    let high_int: u32 = high_integer_type_casting(5);
    println!("High int: {}", high_int);
    let low_int: u8 = low_integer_type_casting(5);
    println!("Low int: {}", low_int);
    let my_string = String::from("Hello, World");
    let my_str = convert_string_to_str(&my_string);
    println!("My string: {}", my_str);
    let my_str = "Hello, World";
    let my_string = convert_str_to_string(my_str);
    println!("My string: {}", my_string);
    let (addition, substraction, multiplication, division) = arithmetic_signed_ops(10, 5);
    println!("Addition: {}", addition);
    println!("Substraction: {}", substraction);
    println!("Multiplication: {}", multiplication);
    println!("Division: {}", division);
    
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


fn high_integer_type_casting(low_int: u8)-> u32{
    let low_int: u32 = low_int.try_into().unwrap();
    return low_int;
}


fn low_integer_type_casting(high_int: u32)-> u8{
    let high_int: u8 = high_int.try_into().unwrap();
    return high_int;
}

fn convert_string_to_str(my_string: &String) -> &str {
    my_string.as_str()
}

fn convert_str_to_string(my_str: &str) -> String {
    my_str.to_string()
}

fn arithmetic_signed_ops(first: i32, second: i32) -> (i32, i32, i32, i32) {
    let addition: i32 = first + second;
    let substraction: i32 = first - second;
    let multiplication: i32 = first * second;
    let division: i32 = first / second;
    return (addition, substraction, multiplication, division);
}