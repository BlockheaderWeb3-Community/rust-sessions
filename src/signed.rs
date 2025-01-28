// function to encapsulate all integers
pub fn intro_to_i(){
    let sum_result:i8 = sum_unsign(5, 10);
    let mult_result:i64  = multiply_unsign(5, 10);
    let divide:i64 = divide_unsign(20, 10);
    let subtr:i8 = substract_unsign(20, 10);
    
    println!("Sum: {}", sum_result);
    println!("Multiplication: {}", mult_result);
    println!("Division: {}", divide);
    println!("Substraction: {}", subtr);    
}

fn sum_unsign(x: i8, y: i8) -> i8 {
    x + y
}
fn multiply_unsign(x: i64, y: i64) -> i64 {
    x * y
}
fn divide_unsign(x: i64, y: i64) -> i64 {
    let res: i64 = x / y;
    return res
}
fn substract_unsign(x: i8, y: i8) -> i8 {
    x - y
}
