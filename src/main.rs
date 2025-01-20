
// Example usage
fn main() {
    // Integer conversion examples
    let small_num: u8 = 255;
    let big_num = u8_to_u32(small_num);
    println!("Converted u8 to u32: {}", big_num);

    let large_num: u32 = 256;
    match u32_to_u8(large_num) {
        Some(num) => println!("Converted u32 to u8: {}", num),
        None => println!("Value {} is too large for u8", large_num),
    }

    // String conversion example
    let owned_string = String::from("Hello, World!");
    let str_ref = string_as_str(&owned_string);
    println!("String as str: {}", str_ref);

    // Arithmetic example
    let a = 100;
    let b = 20;
    if let Some((sum, diff, prod, quot)) = checked_arithmetic(a, b) {
        println!(
            "Sum: {}, Difference: {}, Product: {}, Quotient: {}",
            sum, diff, prod, quot
        );
    }
}

// Convert from low integer to high integer (u8 -> u32)
fn u8_to_u32(value: u8) -> u32 {
    value as u32
}

// Convert from high bit to low bit with overflow checking
fn u32_to_u8(value: u32) -> Option<u8> {
    if value <= u8::MAX as u32 {
        Some(value as u8)
    } else {
        None
    }
}

// Convert String to &str (Safe string to str conversion that doesn't leak memory)
fn string_as_str(s: &String) -> &str {
    s.as_str()
}

// Arithmetic operations on signed integers with overflow checking
fn checked_arithmetic(a: i32, b: i32) -> Option<(i32, i32, i32, i32)> {
    // Returns (sum, difference, product, quotient)
    let sum = a.checked_add(b)?;
    let difference = a.checked_sub(b)?;
    let product = a.checked_mul(b)?;
    let quotient = if b != 0 {
        a.checked_div(b)?
    } else {
        return None;
    };

    Some((sum, difference, product, quotient))
}
