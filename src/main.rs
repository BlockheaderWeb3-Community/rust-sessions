fn main() {
    intro_to_u();
}

fn intro_to_u() {
    let sum_result: u8 = sum(5, 10);
    println!("sum_result is: {}", sum_result);
    let subtract_result: u8 = subtract(10, 5);
    println!("subtract_result is: {}", subtract_result);
    let multiply_result: u8 = multiply(5, 10);
    println!("multiply_result is: {}", multiply_result);
    let divide_result: u8 = divide(10, 5);
    println!("divide_result is: {}", divide_result);
}

fn sum(a: u8, b: u8) -> u8 {
    a + b //implicit return
    //return a + b; //explicit return
}

fn subtract(a: u8, b: u8) -> u8 {
    a - b
}

fn multiply(a: u8, b: u8) -> u8 {
    a * b
}

fn divide(a: u8, b: u8) -> u8 {
    a / b
}