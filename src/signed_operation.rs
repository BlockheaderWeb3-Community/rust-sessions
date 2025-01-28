use crate::utils::{add_inum, div_inum, is_even_num, mul_inum, sub_inum};

pub fn intro_to_i() {
    let add_inum_results: i16 = add_inum(8, 2); // Calculate the sum of two signed integers
    println!("The signed addition result is: {}", add_inum_results);

    let sub_inum_results: i16 = sub_inum(10, 5); // Calculate the difference of two signed integers
    println!("The signed subtraction result is: {}", sub_inum_results);

    let mul_inum_results: i16 = mul_inum(10, 5); // Calculate the product of two signed integers
    println!("The signed multiplication result is: {}", mul_inum_results);

    let div_inum_results: i16 = div_inum(10, 5); // Divide two signed integers
    println!("The signed division result is: {}", div_inum_results);

    let iseven_inum_results: bool = is_even_num(10, 4); // Check if the sum of two numbers is even
    println!("The signed even result is: {}", iseven_inum_results); 
}




 