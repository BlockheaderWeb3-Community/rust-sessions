fn main() {
    println!(
        "Is Even? {}. The sum of the numbers, {} and {}.",
        is_even(5, 5),
        5,
        5,
    );
    println!(
        "Is Even? {}. The sum of the numbers, {} and {}.",
        is_even(22, 21),
        22,
        21,
    );
    intro_to_f();
    say_name("Eleazar", "Anonefdora");
}

// is sum even?
fn is_even(x: i32, y: i32) -> bool {
    let z: i32 = sum_even(x, y);
    if z % 2 == 0 {
        true
    } else {
        false
    }
}

fn sum_even(x: i32, y: i32) -> i32 {
    x + y
}

// floating (arithmetic function)
fn intro_to_f() {
    let sum_result: f32 = sum(5.02, 10.02);
    println!("The sum is {:?}", sum_result);

    let subtract: f32 = subtract(15.22, 10.02);
    println!("The subtraction is {:?}", subtract);

    let divide: f32 = divide(15.02, 10.41);
    println!("The division is {:?}", divide);

    let multiply: f32 = multiply(15.90, 10.02);
    println!("The multiplication is {:?}", multiply);

    let module: f32 = modulo(15.12, 10.02);
    println!("The modulo is {:?}", module);
}

fn sum(x: f32, y: f32) -> f32 {
    x + y
}

fn subtract(x: f32, y: f32) -> f32 {
    x - y
}

fn divide(x: f32, y: f32) -> f32 {
    x / y
}

fn multiply(x: f32, y: f32) -> f32 {
    x * y
}

fn modulo(x: f32, y: f32) -> f32 {
    x % y
}

// String concatenation
fn say_name(first_name: &str, surname: &str) {
    let full_name = format!("Hello, {} {}", first_name, surname);
    println!("{}", full_name);
}
