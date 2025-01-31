pub fn arithmetic_signed_ops(first: i32, second: i32) -> (i32, i32, i32, i32) {
    let addition: i32 = first + second;
    let substraction: i32 = first - second;
    let multiplication: i32 = first * second;
    let division: i32 = first / second;
    return (addition, substraction, multiplication, division);
}

pub fn arithmetic_unsigned_ops(first: u32, second: u32) -> (u32, u32, u32, u32) {
    let addition: u32 = first + second;
    let substraction: u32 = first - second;
    let multiplication: u32 = first * second;
    let division: u32 = first / second;
    return (addition, substraction, multiplication, division);
}

pub fn arithmetic_floatingpoint_ops(first: f64, second: f64) -> (f64, f64, f64, f64) {
    let addition = first + second;
    let substraction = first - second;
    let multiplication = first * second;
    let divide = first / second;
    (addition, substraction, multiplication, divide)
}

pub fn integer_type_casting(first_int: u32, second_int: u8) -> (u8, u32) {
    (
        first_int.try_into().unwrap(),
        second_int.try_into().unwrap(),
    )
}
