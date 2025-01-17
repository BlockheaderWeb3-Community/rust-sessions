fn main() {
    intro_to_u();
}

fn intro_to_u() {
    let sum_result: u8 = sum(5, 4);
    let sub_result: u8 = subtract(10, 5);
    let multiply_result: u8 = multiply(10, 5);
    let divide_result: u8 = divide(10, 5);
    let is_even_result: bool = is_even_sum(5, 5);
    let is_even : bool = is_even(8);
    let full_name = fullname("Uche", "Caleb", "Solomon");

    println!("The sum result is: {}", sum_result);
    println!("The subtraction result is: {}", sub_result);
    println!("The multiplication result is: {}", multiply_result);
    println!("The division result is: {}", divide_result);
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

/// Function to check if the sum of two numbers is even
fn is_even_sum(x: u8, y: u8) -> bool {
    let sum = x + y;
    sum % 2 == 0
}

fn is_even(x: u8,) -> bool{
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