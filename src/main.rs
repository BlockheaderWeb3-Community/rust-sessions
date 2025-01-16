fn main() {
    intro_to_u();
}


// function to encapsulate all integers
fn intro_to_u() {
    let sum_result: u8 = sum(5, 10);
    println!("the sum result is: {}", sum_result);
    let sub_result: u16 = sub(8, 8);
    println!("the sub result is: {}, sub_result");
    let mul_result: u16 = mul(2, 4);
    println!("the mul result is: {}", mul_result);
    let divide_result: u16 = divide(10, 10);
    println!("the divide result is: {}", divide_result);   

}

fn sum(x: u8, y: u8) -> u8 {
    x + y // implicit return
//    return x + y; // explicit return
}

// subtract
fn sub(x: u16, y: u16) -> u16 {
    x - y
}
// multiplication
fn mul(x: u16, y: u16) -> u16 {
    x * y
}

// division
fn divide(x: u16, y: u16) -> u16 {
    x / y
}

