mod primitive_data_types;

fn main() {
    intro_to_u();
    primitive_data_types::intro_to_us();
}


// function to encapsulate all integers
fn intro_to_u() {
    let sum_result: u8 = sum(5, 10);
    println!("the sum result is: {}", sum_result);   


}

fn sum(x: u8, y: u8) -> u8 {
    x + y // implicit return
//    return x + y; // explicit return
}


// subtract
// multiplication
// division


