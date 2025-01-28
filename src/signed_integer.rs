pub fn signed_int() {
    let result_sum: i32 = sum(10, 7).into();
    let result_substrt: i32 = substraction(23, 17);
    let result_division: i32 = division(25, 5);
    let result_multiply: i32 = multiply(23, 17);

    println!("The sum result is {}", result_sum);
    println!("The result is {}", result_substrt);
    println!("The result is {}", result_division);
    println!("The result is {}", result_multiply);
}

pub fn sum(x: i32, y: i32) -> i32 {
    x + y
    // return x + y; // explicit return
}

pub fn substraction(x: i32, y: i32) -> i32 {
    x - y
}

pub fn division(x: i32, y: i32) -> i32 {
    x / y
}

pub fn multiply(x: i32, y: i32) -> i32 {
    x * y
}