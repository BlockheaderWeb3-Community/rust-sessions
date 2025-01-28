//mod strings;
mod unsigned;
mod signed;
mod float;
mod string;
fn main() {
    unsigned::intro_to_u();
    signed::intro_to_i();
    float::intro_to_float();
    string::strings();
    //intro_to_u();
    // println!("------------string handler-------------");
    // string_handler();
    // println!("------------number data type conversion-------------");
    // nunber_data_type_converter();
    // println!("------------string data type conversion-------------");
    // string_data_type_converter();
    // println!("------------signed number-------------");
    // intro_to_i();
}




// fn check_func(num1: u8, num2: u8) -> bool {
//     let sum_of_two_nums = sum(num1, num2);
//     if sum_of_two_nums % 2 == 0 {
//         println!("The sum of {} and {} is even", num1, num2);
//         return true;
//     } else {
//         println!("The sum of {} and {} is odd", num1, num2);
//         return false;
//     }
// }
