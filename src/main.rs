fn main() {
    intro_to_u();
}

fn intro_to_u() {
    let sum_result: u8 = sum(5, 10);
    println!("The sum is {}", sum_result);

    let subtract: u8 = subtract(15, 10);
    println!("The subtraction is {}", subtract);

    let divide: u8 = divide(15, 10);
    println!("The division is {}", divide);

    let multiply: u8 = multiply(15, 10);
    println!("The multiplication is {}", multiply);

    let module: u8 = modulo(15, 10);
    println!("The modulo is {}", module);
}

fn sum(x: u8, y: u8) -> u8 {
    x + y
}

fn subtract(x:u8, y: u8) -> u8 {
    x - y
}

fn divide(x:u8, y: u8) -> u8 {
    x / y
}

fn multiply(x:u8, y: u8) -> u8 {
    x * y
}

fn modulo(x:u8, y: u8) -> u8 {
    x % y
}