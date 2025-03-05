//mod strings;
mod collections;
mod constructor;
mod float;
mod signed;
mod string;
mod todo;
mod unsigned;
mod user_struct;

fn create_todo(mut todos: Vec<todo::Todo>) -> Vec<todo::Todo> {
    let tid = todos.len();
    todos.push(todo::Todo::new(
        tid.clone() as u32,
        "New Day".to_string(),
        todo::Status::Pending,
        "22:02:25".to_string(),
        "Lorem Ipsom".to_string(),
    ));

    print!("todo {:#?}", todos[tid]);

    todos
}

fn main() {
    unsigned::intro_to_u();
    signed::intro_to_i();
    float::intro_to_float();
    string::strings();
    user_struct::user_registry();
    collections::collections();

    //Book creation
    let book = constructor::Book::new("The Rust Programming Language", "Steve Klabnik", 2019);

    println!("Title: {}", book.title);
    println!("Author: {}", book.author);
    println!("Year: {}", book.year);
    println!("Likes: {}", book.likes);

    let mut todo_task: Vec<todo::Todo> = Vec::new();

    todo_task = create_todo(todo_task);
    todo_task = create_todo(todo_task);
    todo_task = create_todo(todo_task);
}
