pub fn unsigned_int() {
    let result_sum: u32 = sum(10, 7).into();
    let result_substrt: u32 = substraction(23, 17);
    let result_division: u32 = division(25, 5);
    let result_multiply: u32 = multiply(23, 17);

    println!("The sum result is {}", result_sum);
    println!("The result is {}", result_substrt);
    println!("The result is {}", result_division);
    println!("The result is {}", result_multiply);
}

pub fn sum(x: u32, y: u32) -> u32 {
    x + y
    // return x + y; // explicit return
}

pub fn substraction(x: u32, y: u32) -> u32 {
    x - y
}

pub fn division(x: u32, y: u32) -> u32 {
    x / y
}

pub fn multiply(x: u32, y: u32) -> u32 {
    x * y
}