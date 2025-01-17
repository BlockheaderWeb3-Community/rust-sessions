use core::convert::Into;

pub fn intro_to_us() {
    let result_sum: u16 = sum(10, 7).into();
    let result_substrt: u16 = substraction(23, 17);
    let result_division: u16 = division(25, 5);
    let result_multiply: u16 = multiply(23, 17);
    let is_even: bool = is_even(22, 2);
    let is_sum_even: bool = is_sum_even(21, 2);
    let floating_sum: f32 = floating_sum(3.6, 8.8);
    let floating_substrt: f32 = floating_substrt(24.6, 8.8);
    let make_string: String = make_string("martin", "19");

    println!("The sum result is {}", result_sum);
    println!("The result is {}", result_substrt);
    println!("The result is {}", result_division);
    println!("The result is {}", result_multiply);
    println!("is_even result is {}", is_even);
    println!("is_even result is {}", is_sum_even);
    println!("floating_sum result is {}", floating_sum);
    println!("floating_sum result is {}", floating_substrt);
    println!("my name is {}", make_string);

    intro_to_ownable_string();
}

pub fn sum(x: u8, y: u8) -> u8 {
    x + y
    // return x + y; // explicit return
}

pub fn substraction(x: u16, y: u16) -> u16 {
    x - y
}

pub fn division(x: u16, y: u16) -> u16 {
    x / y
}

pub fn multiply(x: u16, y: u16) -> u16 {
    x * y
}

pub fn is_even(x: u16, y: u16) -> bool {
    if x % y == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn is_sum_even(x: u32, y: u32) -> bool {
    let is_sum_evens: u32 = x + y;
    if is_sum_evens % 2 == 0 {
        true
    } else {
        false
    }
}

pub fn floating_sum(x: f32, y: f32) -> f32 {
    x + y
}

pub fn floating_substrt(x: f32, y: f32) -> f32 {
    x - y
}

pub fn make_string(x: &str, y: &str) -> String {
    format!("{x} and i scored {y}")
}

pub fn intro_to_ownable_string() {
    let mut fname: String = String::from("martin");
    println!("first name: {} {}", fname, fname.len());

    fname.push_str("vibes");
    println!("first name_______: {} {}", fname, fname.len());
}