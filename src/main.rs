
fn main() {
    intro_to_u();
}


// function to encapsulate all integers
fn intro_to_u() {
    let sum_result: u16 = sum(5, 10);
    println!("the sum result is: {}", sum_result);
    let sub_result: u16 = sub(8, 8);
    println!("the sub result is: {}", sub_result);
    let mul_result: u16 = mul(2, 4);
    println!("the mul result is: {}", mul_result);
    let divide_result: u16 = divide(10, 10);
    println!("the divide result is: {}", divide_result);
    let sum_even_result: u16 = sum_even_num(10, 5);
    println!("the sum to evem result is {}", sum_even_result ); 
    let is_even_result: bool = is_even_num(10, 5);
    println!("the is even num is {}", is_even_result ); 

}

fn sum(x: u16, y: u16) -> u16 {
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

fn sum_even_num(x: u16,y: u16) -> u16 {
    x + y
}

fn is_even_num(x: u16, y: u16) -> bool {
    let z: u16 = sum_even_num(x, y);
        if (z) % 2 ==0 {
        true
    } else {
        false
    }
}

