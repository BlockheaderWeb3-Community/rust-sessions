pub mod arithmetic_ops;
pub mod check_func;
pub mod string_ops;
pub mod calculator;

mod float;
mod signed;
mod string;
mod unsigned;

fn main() {
    // intro_to_rust::intro_to_rust();
    // intro_to_rust::print_arithmetic_ops_results();
    // intro_to_rust::print_string_ops_results();
    // intro_to_rust::check_func_results();
  
//       unsigned::intro_to_u();
//     signed::intro_to_i();
//     float::intro_to_float();
//     string::strings();
  
    calculator::main();
}
pub mod intro_to_rust {
    use crate::{
        arithmetic_ops::{
            arithmetic_floatingpoint_ops, arithmetic_signed_ops, arithmetic_unsigned_ops,
            integer_type_casting,
        },
        check_func::check_func,
        string_ops::{
            intro_to_str_slice, string_conversion_ops, string_formatting, string_handler,
        },
    };

    pub fn intro_to_rust() {
        println!("Welcome to Rust Programming Language");
    }

    pub fn print_arithmetic_ops_results() {
        println!("The Sum, substraction, multiplication and division of the two numbers passed in this function are   {:?}", arithmetic_signed_ops(20, 3));
        println!(
            "This function types casts the first u32 to u8 amd the second u8 to u32 {:?} ",
            integer_type_casting(20, 3)
        );
        println!("The Sum, substraction, multiplication and division of the two numbers passed in this function are    {:?}", arithmetic_unsigned_ops(20, 3));
        println!("The Sum, substraction, multiplication and division of the two numbers passed in this function are    {:?}", arithmetic_floatingpoint_ops(2.0, 3.0));
    }

    pub fn print_string_ops_results() {
        let first_name = String::from("AKinshola");
        let last_name = String::from("Akinniyi");
        println!(
            "The full name of the person is {}",
            string_formatting(first_name, last_name)
        );
        println!(
            "The string literal and the main string are {:?}",
            string_conversion_ops("Hello", &String::from("World"))
        );
        println!("The string slice is {:?}", intro_to_str_slice());
        string_handler();
    }
    pub fn check_func_results() {
        println!(" {:?} ", check_func(8, 10));
    }
}
