
fn main() {
    intro_to_u();
    intro_to_i();
    intro_to_bool();
    intro_to_string();
}

fn intro_to_u() {

    let sum_result_for_unsigned_integer: u8 = sum_for_unsigned_integer(5, 10);
    let subtraction_result_for_unsigned_integer: u8 = subtraction_for_unsigned_integer(10, 5);
    let multiplication_result_for_unsigned_integer: u8 = multiplication_for_unsigned_integer(5, 10);
    let division_result_for_unsigned_integer: f32 = division_for_unsigned_integer(10.0, 3.0);

    let u8int_to_u32int = convert_u8int_to_u32int(200);
    let u32int_to_u8int = convert_u32int_to_u8int(100);

    println!("The sum is (Unsigned Int): {}", sum_result_for_unsigned_integer);
    println!("The difference is (Unsigned Int): {}", subtraction_result_for_unsigned_integer);
    println!("The multiplication is (Unsigned Int): {}", multiplication_result_for_unsigned_integer);
    println!("The division is (Unsigned Int): {}", division_result_for_unsigned_integer);

    println!("u8 to u3 conversion: {}", u8int_to_u32int);
    println!("u32 to u8 conversion: {}", u32int_to_u8int);

}

fn intro_to_i(){

    let sum_result_for_signed_integer: i8 = sum_for_signed_integer(5, 10);
    let subtraction_result_for_signed_integer: i8 = subtraction_for_signed_integer(10, 5);
    let multiplication_result_for_signed_integer: i8 = multiplication_for_signed_integer(5, 10);
    let division_result_for_signed_integer: i8 = division_for_signed_integer(10, 2);

    println!("The sum is (Signed Int): {}", sum_result_for_signed_integer);
    println!("The difference is (Signed Int): {}", subtraction_result_for_signed_integer);
    println!("The multiplication is (Signed Int): {}", multiplication_result_for_signed_integer);
    println!("The division is (Signed Int): {}", division_result_for_signed_integer);

}

fn intro_to_bool(){
    let is_sum_even: bool = is_sum_even(10,5);
    println!("Is Sum even?: {}", is_sum_even);
}

fn intro_to_string(){
    let name: String = display_name("Jethro".to_string(), "Adamu".to_string());
    let reg_number: String = String::from("HG-2025-001");
    let reg_number_to_string: &str = convert_string_to_str(&reg_number);

    println!("Your name is: {}", name);
    println!("Registration number as String slice: {}", reg_number_to_string);
}

/////////////////////////////////////////////////////////////////////////////////////////
/********** This Block handles arithmetic operations for unsigned integers *************/
/////////////////////////////////////////////////////////////////////////////////////////
fn sum_for_unsigned_integer(x:u8, y:u8) -> u8 {
    x + y
}

fn subtraction_for_unsigned_integer(x:u8, y:u8) -> u8 {
    x - y
}

fn multiplication_for_unsigned_integer(x:u8, y:u8) -> u8 {
    x * y
}

fn division_for_unsigned_integer(x:f32, y:f32) -> f32 {
    x / y
}

/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////////////
/********** This Block handles arithmetic operations for signed integers *************/
/////////////////////////////////////////////////////////////////////////////////////////
fn sum_for_signed_integer(x:i8, y:i8) -> i8 {
    x + y
}

fn subtraction_for_signed_integer(x:i8, y:i8) -> i8 {
    x - y
}

fn multiplication_for_signed_integer(x:i8, y:i8) -> i8 {
    x * y
}

fn division_for_signed_integer(x:i8, y:i8) -> i8 {
    x / y
}
/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

fn is_sum_even(x:u8, y:u8) -> bool {
    let check_result = sum_for_unsigned_integer(x, y);
    if check_result % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn display_name(first_name: String, last_name: String) -> String {
    return format!("{} {}", first_name, last_name);
}

/////////////////////////////////////////////////////////////////////////////////////////
/******************* This Block handles all conversion function ************************/
/////////////////////////////////////////////////////////////////////////////////////////

fn convert_u8int_to_u32int(u8int: u8) -> u32{
    return u8int.into();
}

fn convert_u32int_to_u8int(u32int: u32) -> u8 {
    return u32int.try_into().expect("The return value is out of range for u8");
}

fn convert_string_to_str(string_name: &String) -> &str {
    return string_name.as_str();
}