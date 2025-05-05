mod session_three;
use session_three::student_registry::student_registry;
// use rust_sessions::cache::Data;
// use std::io;
// use std::io::Write;

// mod lib;

//mod strings;
// mod cache;
// mod calculator;
// mod collections;
// mod constructor;
// mod float;
// mod signed;
// mod string;
// mod todo;
// mod unsigned;
// mod user_struct;
mod session_one;
use session_one::{enums, structs};

// fn read_line() -> String {
//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer).expect("Failed to read");
//     buffer.trim().to_string()
// }

// fn read_key() -> u32 {
//     loop {
//         print!("Enter key (u32): ");
//         // io::stdout().flush().unwrap();
//         let input = read_line();
//         if let Ok(num) = input.parse::<u32>() {
//             return num;
//         } else {
//             println!("Invalid input. Please enter a valid u32.");
//         }
//     }
// }

// fn read_value() -> String {
//     print!("Enter value (String): ");
//     io::stdout().flush().unwrap();
//     read_line()
// }

// fn create_todo(mut todos: Vec<todo::Todo>) -> Vec<todo::Todo> {
//     let tid = todos.len();
//     todos.push(todo::Todo::new(
//         tid.clone() as u32,
//         "New Day".to_string(),
//         todo::Status::Pending,
//         "22:02:25".to_string(),
//         "Lorem Ipsom".to_string(),
//     ));

//     print!("todo {:#?}", todos[tid]);

//     todos
// }

fn main() {
    // unsigned::intro_to_u();
    // signed::intro_to_i();
    // float::intro_to_float();
    // string::strings();
    // user_struct::user_registry();
    // collections::collections();
    structs::user_details();
    enums::enums_details();

    // //Book creation
    // let book = constructor::Book::new("The Rust Programming Language", "Steve Klabnik", 2019);

    // println!("Title: {}", book.title);
    // println!("Author: {}", book.author);
    // println!("Year: {}", book.year);
    // println!("Likes: {}", book.likes);

    // let mut todo_task: Vec<todo::Todo> = Vec::new();

    // todo_task = create_todo(todo_task);
    // todo_task = create_todo(todo_task);
    // todo_task = create_todo(todo_task);

    // let mut data = Data::<u32, String>::new();
    // loop {
    //     println!("\n Select an operation");
    //     println!("1. Add");
    //     println!("2. Get");
    //     println!("3. Delete");
    //     println!("4. Update");
    //     println!("5. Quite");

    //     println!("Enter choice: ");
    //     io::stdout().flush().unwrap();

    //     let choice = read_line().trim().to_string();

    //     match choice.as_str() {
    //         "1" => {
    //             let key = read_key();
    //             let value = read_value();
    //             data.add(key, value);
    //             println!("Added successfully.");
    //         }
    //         "2" => {
    //             let key = read_key();
    //             match data.get(key) {
    //                 Some(v) => println!("Value: {}", v),
    //                 None => println!("Key not found."),
    //             }
    //         }
    //         "3" => {
    //             let key = read_key();
    //             let value = read_value();
    //             match data.update(key, value) {
    //                 Ok(_) => println!("Update successful"),
    //                 Err(e) => println!("{}", e),
    //             }
    //         }
    //         "3" => {
    //             let key = read_key();
    //             let value = read_value();
    //             match data.update(key, value) {
    //                 Ok(_) => println!("Updated successfully."),
    //                 Err(e) => println!("{}", e),
    //             }
    //         }
    //         "4" => {
    //             let key = read_key();
    //             match data.delete(key) {
    //                 Ok(v) => println!("Deleted: {}", v),
    //                 Err(e) => println!("{}", e),
    //             }
    //         }

    //         "5" => {
    //             println!("Exiting...");
    //             break;
    //         }
    //         _ => println!("Invalid choice."),
    //     }
    // }
    // let a: f64 = 20.00;
    // let b: f64 = 12.00;
    // println!(
    //     "The summation of {} and {} is {}",
    //     a,
    //     b,
    //     calculator::add(a, b)
    // );

    // println!(
    //     "The difference between {} and {} is {}",
    //     a,
    //     b,
    //     calculator::subtract(a, b)
    // );
    student_registry();
}
