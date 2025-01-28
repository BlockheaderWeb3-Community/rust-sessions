use crate::{string_operations::{string_formating_names, convert_to_string_i1, convert_to_string_i2}, utils::{divide, float_sub, float_sum, is_even_num, mul, sub, sum }};

// Function to encapsulate operations on unsigned integers
pub fn intro_to_u() {
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


