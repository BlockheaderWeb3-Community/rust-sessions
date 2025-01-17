
fn main() {
    intro_to_u()
}

fn intro_to_u() {
    let sum_result: u8 = sum(5, 10);
    let subtraction_result: u8 = subtraction(10, 5);
    let multiplication_result: u8 = multiplication(5, 10);
    let division_result: f32 = division(10.0, 3.0);
    let is_sum_even: bool = is_sum_even(10,5);
    let name: String = display_name("Jethro".to_string(), "Adamu".to_string());

    println!("The sum is: {}", sum_result);
    println!("The difference is: {}", subtraction_result);
    println!("The multiplication is: {}", multiplication_result);
    println!("The division is: {}", division_result);
    println!("Is Sum even?: {}", is_sum_even);
    println!("Your name is: {}", name);
}

fn sum(x:u8, y:u8) -> u8 {
    x + y
}

fn is_sum_even(x:u8, y:u8) -> bool {
    let check_result = sum(x, y);
    if check_result % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn subtraction(x:u8, y:u8) -> u8 {
    x - y
}

fn multiplication(x:u8, y:u8) -> u8 {
    x * y
}

fn division(x:f32, y:f32) -> f32 {
    x / y
}

fn display_name(first_name: String, last_name: String) -> String {
    return format!("{} {}", first_name, last_name);
}