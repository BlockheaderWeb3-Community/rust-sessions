fn main() {
    intro_to_u(); // Introduction to unsigned integer operations
    intro_to_i(); // Introduction to signed integer operations
}

// Function to encapsulate operations on unsigned integers
fn intro_to_u() {
    let sum_result: u16 = sum(5, 10); // Calculate the sum of two unsigned integers
    println!("The sum result is: {}", sum_result);

    let sub_result: u16 = sub(8, 8); // Calculate the difference of two unsigned integers
    println!("The subtraction result is: {}", sub_result);

    let mul_result: u16 = mul(2, 4); // Calculate the product of two unsigned integers
    println!("The multiplication result is: {}", mul_result);

    let divide_result: u16 = divide(10, 10); // Divide two unsigned integers
    println!("The division result is: {}", divide_result);

    let is_even_result: bool = is_even_num(10, 5); // Check if the sum of two numbers is even
    println!("The result of is_even_num is: {}", is_even_result); 

    let float_sum_result: f32 = float_sum(10.0, 5.0); // Calculate the sum of two floating-point numbers
    println!("The float sum result is: {}", float_sum_result); 

    let float_sub_result: f32 = float_sub(10.0, 4.0); // Calculate the difference of two floating-point numbers
    println!("The float subtraction result is: {}", float_sub_result);

    let combine_names_result = string_formating_names(
        &convert_to_string_i1("Oluwaseun"), // Convert first name to a string
        &convert_to_string_i2("Jeremiah")  // Convert last name to a string
    );
    println!("Hello, my name is: {}", combine_names_result);
}

// Function to add two unsigned integers
fn sum(x: u16, y: u16) -> u16 {
    x + y // Implicit return of the sum
}

// Function to subtract one unsigned integer from another
fn sub(x: u16, y: u16) -> u16 {
    x - y
}

// Function to multiply two unsigned integers
fn mul(x: u16, y: u16) -> u16 {
    x * y
}

// Function to divide two unsigned integers, panicking if the divisor is 0
fn divide(x: u16, y: u16) -> u16 {
    if y == 0 {
        panic!("Number not divisible by 0");
    }
    x / y
}

// Function to check if the sum of two unsigned integers is even
fn is_even_num(x: u16, y: u16) -> bool {
    let sum_two_even_number: u16 = sum(x, y);
    sum_two_even_number % 2 == 0 // Return true if the sum is even, otherwise false
}

// Function to calculate the sum of two floating-point numbers
fn float_sum(x: f32, y: f32) -> f32 {
    x + y
}

// Function to calculate the difference of two floating-point numbers
fn float_sub(x: f32, y: f32) -> f32 {
    x - y
}

// Function to format two strings as a full name
fn string_formating_names(first_name: &str, last_name: &str) -> String {
    format!("{} {}", first_name, last_name) // Concatenate first and last names
}

// Function to convert a string slice to a `String` using `to_string`
fn convert_to_string_i1(x: &str) -> String {
    x.to_string()
}

// Function to convert a string slice to a `String` using `String::from`
fn convert_to_string_i2(x: &str) -> String {
    String::from(x)
}

// Function to encapsulate operations on signed integers
fn intro_to_i() {
    let add_inum_results: i16 = add_inum(8, 2); // Calculate the sum of two signed integers
    println!("The signed addition result is: {}", add_inum_results);

    let sub_inum_results: i16 = sub_inum(10, 5); // Calculate the difference of two signed integers
    println!("The signed subtraction result is: {}", sub_inum_results);

    let mul_inum_results: i16 = mul_inum(10, 5); // Calculate the product of two signed integers
    println!("The signed multiplication result is: {}", mul_inum_results);

    let div_inum_results: i16 = div_inum(10, 5); // Divide two signed integers
    println!("The signed division result is: {}", div_inum_results);

    let iseven_inum_results: bool = is_even_num(10, 4); // Check if the sum of two numbers is even
    println!("The signed even result is: {}", iseven_inum_results); 
}

// Function to add two signed integers
fn add_inum(x: i16, y: i16) -> i16 {
    x + y
}

// Function to subtract one signed integer from another
fn sub_inum(x: i16, y: i16) -> i16 {
    x - y
}

// Function to multiply two signed integers
fn mul_inum(x: i16, y: i16) -> i16 {
    x * y
}

// Function to divide two signed integers, panicking if the divisor is 0
fn div_inum(x: i16, y: i16) -> i16 {
    if y == 0 {
        panic!("Number not divisible by 0");
    }
    x / y
}

// Function to return the larger integer value by converting the sum to `u32`
fn lowinteger_highinteger(x: u16, y: u16) -> u32 {
    (x + y) as u32
}
