fn main() {
    intro_to_u();
    intro_to_f();
    intro_to_char()
}


// function to encapsulate all integers
fn intro_to_u() {
    let sum_result: u8 = sum(5, 20);
    println!("the sum result is: {}", sum_result);  
    
    let subtract_result: u8 = subtract(20, 15);
    println!("the subtract result is: {}", subtract_result);

    let multiply_result: u8 = multiply(5, 12);
    println!("the multiply result is {}", multiply_result);

    let divide_result: u8 = divide(20, 4);
    println!("the divide result is {}", divide_result);



}

// functions for Floating points

fn intro_to_f() {
    let add_result: f64 = add(10.3.into(), 10.1.into()); 
    println!("the add result is: {}", add_result);


let minus_result: f64 = minus(25.5.into(), 5.5.into());
println! ("the subract2 result is: {}", minus_result);

let rise_result: f64 = rise(5.2.into(), 3.7.into() );
println!("the multiply2 result is {}", rise_result);


let split_result: f64 = split(30.9.into(), 2.3.into());
println!("the divide result is {}", split_result);





}


// function char


fn intro_to_char() {
let name_result: String = name('D', 'o'); 
println!("The char result is: {}", name_result);

let myname_result: String = myname("Darey", "Olowo");
 println!("my name are : {}", myname_result);


}


// function to encapsulate all integers

fn sum(x: u8, y: u8) -> u8 {
    x + y // implicit return
//    return x + y; // explicit return
}


// subtract

fn subtract(x: u8, y: u8) -> u8 {
   return x - y
}


// multiplication

fn multiply(x: u8, y: u8) -> u8 {
    x * y
}
// division

fn divide(x: u8, y: u8) -> u8 {
    x / y
}



// Floating-point Types

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn minus(x: f64, y: f64) -> f64 {
    x - y
}

fn rise(x: f64, y: f64) -> f64 {
    x * y
}

fn split(x: f64, y: f64) -> f64 {
    x / y
}


// char type 

fn name(x: char, y: char) -> String { 
    let mut  name = String::new(); 
    name.push(x); 
    name.push(y); 
    name }


    fn myname(x: &str, y: &str) -> String {
         let mut myname = String::new(); 
         myname.push_str(x); 
         myname.push_str(y); 
         myname }