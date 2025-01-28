// function to encapsulate all floatin point
pub fn intro_to_float(){
    //let check: bool = check_func(5, 10);

    let sum_result: f64 = sum_fp(5.0, 10.0);
    let mult_result: f64 = multiply_fp(5.0, 10.0);
    let divide: f64 = divide_fp(20.0, 10.2);
    let subtr: f64 = substract_fp(20.0, 10.0);
    println!("Sum: {}", sum_result);
    println!("Multiplication: {}", mult_result);
    println!("Division: {}", divide);
    println!("Substraction: {}", subtr);

    // let full_name = string_formatting(convert_to_string_v1("Abdulmajid"), convert_to_string_v2("Yunus"));
    // println!("Full Name: {}", full_name);
}

fn sum_fp(x: f64, y: f64) -> f64 {
    x + y
}

fn multiply_fp(x: f64, y: f64) -> f64 {
    x * y
}

fn divide_fp(x: f64, y: f64) -> f64 {
    x / y
}

fn substract_fp(x: f64, y: f64) -> f64 {
    x - y
}