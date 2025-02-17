#[derive(Debug)]
struct User{
    id:u64,
    name:String,
    gender:Sex,
    marital_status:Status,
}

#[derive(Debug)]
enum Status {
    Married,
    Single,
    Divorce,
    Widow
}

#[derive(Debug)]
enum Sex{
    Male,
    Female
}

impl User {
    fn new_user(id:u64,name:String,gender:Sex,marital_status:Status) -> User{
        User {
            id,
            name,
            gender,
            marital_status,
        }
    }
 
    fn update_user_name(name:String, users_data: &mut Vec<User>, id:u64){
        let updated_name = users_data.iter_mut().find(|x| x.id == id ).unwrap();
        updated_name.name = name;
    }
    fn update_user_sex(gender: Sex,users_data: &mut Vec<User>, id:u64){
        let updated_sex = users_data.iter_mut().find(|x| x.id == id ).unwrap();
        updated_sex.gender=gender;
    }
    fn update_user_marital_status(status:Status,users_data: &mut Vec<User>, id:u64){
        let updated_status = users_data.iter_mut().find(|x| x.id == id ).unwrap();
        updated_status.marital_status=status;
    }
}

fn main() {
    // intro_to_u();
    //string_handler();
    let mut users_data :Vec<User>=  Vec::new();
    users_data.push(User::new_user(5, "yunus".to_string(), Sex::Male, Status::Single));
    users_data.push(User::new_user(4, "Titilola".to_string(), Sex::Female, Status::Divorce));
    users_data.push(User::new_user(3, "yunus".to_string(), Sex::Male, Status::Married));
    users_data.push(User::new_user(2, "funke".to_string(), Sex::Female, Status::Widow));

    User::update_user_name("Iliya".to_string(),&mut users_data,5);
    User::update_user_marital_status(Status::Married,&mut users_data,4);
    User::update_user_sex(Sex::Female,&mut users_data,3);
    User::update_user_name("kemi".to_string(),&mut users_data,2);
 

    //User::update_user_name(&mut user_one, "Abdul".to_string());

    //let q = users_data.iter().find(|&x| x.id == 5 );

    println!("user {:#?}", users_data);
}

#[allow(dead_code)]
// function to encapsulate all integers
fn intro_to_u(){
    let sum_result: u8 = sum(5, 10);
    let mult_result: u64 = multiply(5, 10);
    let divide: f64 = divide(20.0, 10.2);
    let subtr: isize = substract(20, 10);
    let check: bool = check_func(5, 10);
    println!("Sum: {}", sum_result);
    println!("Multiplication: {}", mult_result);
    println!("Division: {}", divide);
    println!("Substraction: {}", subtr);    
    println!("Check: {}", check);

    let sum_result: f64 = sumfp(5.0, 10.0);
    let mult_result: f64 = multiply_fp(5.0, 10.0);
    let divide: f64 = divide_fp(20.0, 10.2);
    let subtr: f64 = substract_fp(20.0, 10.0);
    println!("Sum: {}", sum_result);
    println!("Multiplication: {}", mult_result);
    println!("Division: {}", divide);
    println!("Substraction: {}", subtr);

    let full_name = string_formatting(convert_to_string_v1("Akinshola"), convert_to_string_v2("Akinniyi"));
    println!("Full Name: {}", full_name);
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

fn sumfp(x: f64, y: f64) -> f64 {
    x + y
}

fn multiply_fp(x: f64, y: f64) -> f64 {
    x * y
}

fn divide_fp(x: f64, y: f64) -> f64 {
    x / y
}

fn substract_fp(x: f64, y: f64) -> f64 {
    x - y
}

fn check_func(num1: u8, num2: u8) -> bool {
    let sum_of_two_nums = sum(num1, num2);
    if sum_of_two_nums % 2 == 0 {
        println!("The sum of {} and {} is even", num1, num2);
        return true;
    } else {
        println!("The sum of {} and {} is odd", num1, num2);
        return false;
    }
}

// fn string_formatting(first_name: &str, last_name: &str) -> String {
//     let full_name = format!("{} {}", first_name, last_name);
//     return full_name;
// }
fn string_formatting(first_name: String, last_name: String) -> String {
    let full_name = format!("{} {}", first_name, last_name);
    return full_name;
}

// util fn version 1 to convert &str to String 
fn convert_to_string_v1(x: &str) -> String {
    x.to_string()
}

// util fn version 2 to convert &str to String 
fn convert_to_string_v2(x: &str) -> String {
   String::from(x)
}

// // function that encapsulate all integers
// fn intro_to_u() {
//     // subtract
//     // multiplication
//     // division
//     let sum_result: u8 = sum(5, 10);
//     println!("the sum result is: {}", sum_result);
// }

// fn sum(x: u8, y: u8) -> u8 {
//     x + y // implicit return
//     //    return x + y; // explicit return
// }

// handle all string-related functions
fn string_handler() {
    // intro_to_str_slice();
    intro_to_ownable_string();
}

// intro string slice
// for fixed-sized strings
fn intro_to_str_slice() {
    let name: &str = "Sylvia";
    println!("my name is {}", name)
}

fn intro_to_ownable_string() {
    let mut name: String = String::from("Wisdom");
    println!("first name: here: {}", name);
    name.push_str(" John");
    println!("final name: here: {}", name);
    println!("ptr = address in heap memory: {:?}", name.as_ptr());
}



