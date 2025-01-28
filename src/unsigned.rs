// function to encapsulate all integers
pub fn intro_to_u(){
    let sum_result: u8 = sum(5, 10);
    let mult_result: u64 = multiply(5, 10);
    let divide: f64 = divide(20.0, 10.2);
    let subtr: isize = substract(20, 10);
    println!("Sum: {}", sum_result);
    println!("Multiplication: {}", mult_result);
    println!("Division: {}", divide);
    println!("Substraction: {}", subtr);    


    println!("-------- number conversion---------");
    number_data_type_converter();
    let high = low_to_high_nunber_data_type_converter(67);
    let low = high_to_low_nunber_data_type_converter(677);
    println!("high data type to low {low},  low data type to high{high}")
}


fn sum(x: u8, y: u8) -> u8 {
    x + y
}
fn multiply(x: u64, y: u64) -> u64 {
    x * y
}
fn divide(x: f64, y: f64) -> f64 {
    let res: f64 = x / y;
    return res
}
fn substract(x: isize, y: isize) -> isize {
    x - y
}

// function to encapsulate all integers converter
fn number_data_type_converter(){
    let low_to_high = low_to_high_nunber_data_type_converter(3);
    let high_to_low = high_to_low_nunber_data_type_converter(40);
    println!("A u8 signed  integer of {} is converted to u32 signed integer",low_to_high);
    println!("A u8 signed  integer of {} is converted to u32 signed integer",high_to_low);
}

// handle all u8 convertion to u32
fn low_to_high_nunber_data_type_converter(x:u8)->u32{
    x as u32 
}
// handle all u8 convertion to u32
fn high_to_low_nunber_data_type_converter(x:u32)->u8{
    x as u8 
}