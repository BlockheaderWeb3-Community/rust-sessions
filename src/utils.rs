// Function to add two unsigned integers
pub fn sum(x: u16, y: u16) -> u16 {
    x + y // Implicit return of the sum
}

// Function to subtract one unsigned integer from another
pub fn sub(x: u16, y: u16) -> u16 {
    x - y
}

// Function to multiply two unsigned integers
pub fn mul(x: u16, y: u16) -> u16 {
    x * y
}

// Function to divide two unsigned integers, panicking if the divisor is 0
pub fn divide(x: u16, y: u16) -> u16 {
    if y == 0 {
        panic!("Number not divisible by 0");
    }
    x / y
}

// Function to check if the sum of two unsigned integers is even
pub fn is_even_num(x: u16, y: u16) -> bool {
    let sum_two_even_number: u16 = sum(x, y);
    sum_two_even_number % 2 == 0 // Return true if the sum is even, otherwise false
}

// Function to calculate the sum of two floating-point numbers
pub fn float_sum(x: f32, y: f32) -> f32 {
    x + y
}

// Function to calculate the difference of two floating-point numbers
pub fn float_sub(x: f32, y: f32) -> f32 {
    x - y
}

// Function to add two signed integers
pub fn add_inum(x: i16, y: i16) -> i16 {
    x + y
}

// Functions to encapsulate operations on signed integers

// Function to subtract one signed integer from another
pub fn sub_inum(x: i16, y: i16) -> i16 {
    x - y
}

// Function to multiply two signed integers
pub fn mul_inum(x: i16, y: i16) -> i16 {
    x * y
}

// Function to divide two signed integers, panicking if the divisor is 0
pub fn div_inum(x: i16, y: i16) -> i16 {
    if y == 0 {
        panic!("Number not divisible by 0");
    }
    x / y
}

// Function to return the larger integer value by converting the sum to `u32`
pub fn lowinteger_highinteger(x: u16, y: u16) -> u32 {
    (x + y) as u32
}
