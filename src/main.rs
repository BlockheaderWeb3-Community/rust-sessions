fn main() {
    intro_to_u();
    intro_to_i();
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
    let is_even_result: bool = is_even_num(10, 5);
    println!("the is even num is {}", is_even_result ); 
    let float_sum_result: f32 = float_sum(10.0, 5.0);
    println!("the float number result is {}", float_sum_result ); 
    let float_sub_result: f32 = float_sub(10.0, 4.0);
    println!("the float number result is {}", float_sub_result );
    let combine_names_result= string_formating_names(
        &convert_to_string_i1("Oluwaseun"), 
        &convert_to_string_i2("Jeremiah" )
    );
    println!("Hello my name is: {}", combine_names_result);

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
    if y == 0 {
        panic!("number not divisible by 0")
    } 
    x / y
}

fn is_even_num(x: u16, y: u16) -> bool {
    let sum_two_even_number: u16 = sum(x, y);
        if sum_two_even_number % 2 ==0 {
        true
    } else {
        false
    }
}

fn float_sum(x: f32, y: f32) -> f32 {
    x + y
}

fn float_sub(x: f32, y: f32) -> f32 {
    x - y
}

fn _float_mul(x: f32, y: f32) -> f32 {
    x * y
}

fn _float_div(x: f32, y: f32) -> f32 {
    x / y
}
fn string_formating_names(first_name: &str,last_name: &str) -> String {
     format!("{} {}", first_name, last_name)
 }

fn convert_to_string_i1(x: &str) -> String {
    x.to_string()
}

fn convert_to_string_i2(x: &str) -> String {
    String::from(x)
}

fn intro_to_i() {
    let add_inum_results: i16 = add_inum(8, 2);
    println!("The add inumber result: {}", add_inum_results);
    let sub_inum_results: i16 = sub_inum(10, 5);
    println!("The subtraction inumber result: {}", sub_inum_results);
    let mul_inum_results: i16 = mul_inum(10, 5);
    println!("The multiplication inumber result: {}", sub_inum_results);
    let div_inum_results: i16 = div_inum(10, 5);
    println!("The divided inumber result: {}", div_inum_results);
    let iseven_inum_results: bool = is_even_num(10, 4);
    println!("The even inumber result: {}", iseven_inum_results); 
    
}

fn add_inum(x: i16, y: i16) -> i16 {
    x + y
}
fn sub_inum(x: i16,y: i16) -> i16 {
    x - y
}

fn mul_inum(x: i16, y: i16) -> i16 {
    x * y
} 

fn div_inum(x: i16, y: i16) -> i16 {
    if y == 0 {
        panic!("number not dividible be zero");
    } 
    x / y
}

fn iseven_inum(x: i16, y: i16) -> bool {
    let sum_is_evennum:i16 = add_inum(x, y);
    if sum_is_evennum % 2 == 0 {
        true
    } else {
        false
    }

}
