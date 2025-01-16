
fn main() {
    intro_to_u()
}

fn intro_to_u() {
    let sum_result: u8 = sum(5, 10);
    let subtraction_result: u8 = subtraction(10, 5);
    let multiplication_result: u8 = multiplication(5, 10);
    let division_result: u8 = division(10, 5);

    println!("The sum is: {}", sum_result);
    println!("The difference is: {}", subtraction_result);
    println!("The multiplication is: {}", multiplication_result);
    println!("The division is: {}", division_result);
}

fn sum(x:u8, y:u8) -> u8 {
    x + y
}

fn subtraction(x:u8, y:u8) -> u8 {
    x - y
}

fn multiplication(x:u8, y:u8) -> u8 {
    x * y
}

fn division(x:u8, y:u8) -> u8 {
    x / y
}