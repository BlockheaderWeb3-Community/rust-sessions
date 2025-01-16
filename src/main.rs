
fn main() {
   
    intro_to_u();
}

fn intro_to_u(){
let sum_result : u8 = sum(5, 5);
let sub_result : u8 = subtract(10, 5);
let multiply_result : u8 = multiply(10, 5);
let divide_result : u8 = divide(10, 5);
println!("the sum result is: {}", sum_result);
println!("the subtraction result is: {}", sub_result);
println!("the multiplication result is: {}", multiply_result);
println!("the dvision result is: {}", divide_result);
}

fn sum(x: u8, y:u8) -> u8 {
x + y
// return x+y;//explecit return
}

fn subtract(x: u8, y:u8) -> u8 {
x-y
}

fn multiply(x: u8, y:u8) -> u8 {
x*y
}

fn divide(x: u8, y:u8) -> u8 {
x/y
}