
//Implement arithmetic operations on signed integers

pub fn perform_operations() {
    let a: i32 = 25; // A signed 32-bit integer
    let b: i32 = -10; // Another signed 32-bit integer

    // Arithmetic operations
    let sum = a + b;
    let difference = a - b; 
    let product = a * b;
    let quotient = a / b; 
    let remainder = a % b; // Modulus (remainder)

    println!("Arithmetic Operations on signed integers:");
    println!("a = {}, b = {}", a, b);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
}