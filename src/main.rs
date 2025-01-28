mod u8operation;
mod f64operation;
mod char;
mod signed_integers;

fn main() {
    intro_to_u();
    intro_to_f();
    crate::char::intro_to_char();
    convert_high();
    convert_high_to_low();
    crate::signed_integers::perform_operations();
}

// function to encapsulate all integers and to check for even numbers
fn intro_to_u() {
    let sum_result: u8 = u8operation::sum(5, 20);
    println!("the sum result is: {}", sum_result);
    println!(
        "Is the sum result even? {}",
        if  crate::u8operation::is_even(sum_result) { "Yes" } else { "No" }
    );

    let subtract_result: u8 = u8operation::subtract(20, 15);
    println!("the subtract result is: {}", subtract_result);
    println!(
        "Is the subtract result even? {}",
        if crate::u8operation::is_even(subtract_result) {
            "Yes"
        } else {
            "No"
        }
    );

    let multiply_result: u8 = u8operation:: multiply(5, 12);
    println!("the multiply result is {}", multiply_result);
    println!(
        "Is the multiply result even? {}",
        if crate::u8operation::is_even(multiply_result) {
            "Yes"
        } else {
            "No"
        }
    );

    let divide_result: u8 = u8operation::divide(20, 4);
    println!("the divide result is {}", divide_result);
    println!(
        "Is the divide result odd? {}",
        if crate::u8operation::is_odd(divide_result) { "Yes" } else { "No" }
    );
}






// functions for Floating points

fn intro_to_f() {
    let add_result: f64 = f64operation::add(10.3.into(), 10.1.into());
    println!("the add result is: {}", add_result);

    let minus_result: f64 = f64operation::minus(25.5.into(), 5.5.into());
    println!("the subract2 result is: {}", minus_result);

    let rise_result: f64 = f64operation::rise(5.2.into(), 3.7.into());
    println!("the multiply2 result is {}", rise_result);

    let split_result: f64 = f64operation::split(30.9.into(), 2.3.into());
    println!("the divide result is {}", split_result);
}







// how to convert u8 to u32

fn convert_high() {
    let low: u8 = 255;
    let high: u32 = low as u32;
    println!("u8 value: {}, u32 value:{}", low, high);
}
//Function to convert high bit integer to low bit integer u32 to u8

fn convert_high_to_low() {
    let high: u32 = 300;
    let low: u8 = high as u8;
    println!("u32 value: {}, u8 value: {}", high, low);
}







