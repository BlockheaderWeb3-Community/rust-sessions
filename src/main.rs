fn main() {
    intro_to_u();
}

fn intro_to_u() {
    let sum_result: u8 = sum(5, 4);
    let sub_result: u8 = subtract(10, 5);
    let multiply_result: u8 = multiply(10, 5);
    let divide_result: u8 = divide(10, 5);
    let floatsum_result: f64 = floatsum(5.5, 4.4);
    let floatsub_result: f64 = floatsubtract(2.4, 3.5);
    let floatmultiply_result: f64 = floatmultiply(1.0, 5.0);
    let floatdivide_result: f64 = floatdivide(10.4, 5.3);
    let is_even_result: bool = is_even_sum(5, 5);
    let is_even: bool = is_even(8);
    let full_name = fullname("Uche", "Caleb", "Solomon");

    println!("The sum result is: {}", sum_result);
    println!("The subtraction result is: {}", sub_result);
    println!("The multiplication result is: {}", multiply_result);
    println!("The division result is: {}", divide_result);
    println!("The sum result is: {}", floatsum_result);
    println!("The subtraction result is: {}", floatsub_result);
    println!("The multiplication result is: {}", floatmultiply_result);
    println!("The division result is: {}", floatdivide_result);
    println!(
        "The sum of 5 and 5 is even: {}",
        if is_even_result { "true" } else { "false" }
    );
    println!("The number is: {}", is_even);
    println!("{}", full_name);
}

fn sum(x: u8, y: u8) -> u8 {
    x + y
    // return x+y;//explicit return
}

fn subtract(x: u8, y: u8) -> u8 {
    x - y
}

fn multiply(x: u8, y: u8) -> u8 {
    x * y
}

fn divide(x: u8, y: u8) -> u8 {
    x / y
}

// floating point
fn floatsum(x: f64, y: f64) -> f64 {
    x + y
}

fn floatsubtract(x: f64, y: f64) -> f64 {
    x - y
}

fn floatmultiply(x: f64, y: f64) -> f64 {
    x * y
}

fn floatdivide(x: f64, y: f64) -> f64 {
    if y != 0.0 {
        x / y
    } else {
        panic!("Division by zero is not allowed");
    }
}


/// Function to check if the sum of two numbers is even
fn is_even_sum(x: u8, y: u8) -> bool {
    let sum = x + y;
    sum % 2 == 0
}

fn is_even(x: u8) -> bool {
    x % 2 == 0
}

fn fullname(first: &str, second: &str, last: &str) -> String {
    format!(
        "{first} {second} {last}.",
        first = first,
        second = second,
        last = last,
    )
}

