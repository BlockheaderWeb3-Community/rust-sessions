use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug)]
struct Todo {
    title: String,
    description: String,
    due_date: String,
    priority: String,
    status: String,
}

fn main() {
    let mut todos = match load_from_file("storage.txt") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to load todos: {}", e);
            HashMap::new()
        }
    };

    loop {
        println!("\n--- Todo CLI ---");
        println!("1. Create");
        println!("2. Read");
        println!("3. Update");
        println!("4. Delete");
        println!("5. Exit");

        match read_input("Choose an option:") {
            Ok(choice) => match choice.trim() {
                "1" => create_todo(&mut todos),
                "2" => read_todos(&todos),
                "3" => update_todo(&mut todos),
                "4" => delete_todo(&mut todos),
                "5" => {
                    if let Err(e) = save_to_file("storage.txt", &todos) {
                        eprintln!("Error saving todos: {}", e);
                    }
                    println!("Exiting...");
                    break;
                }
                _ => println!("Invalid choice."),
            },
            Err(e) => println!("Input error: {}", e),
        }
    }
}

fn read_input(prompt: &str) -> io::Result<String> {
    print!("{} ", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn create_todo(todos: &mut HashMap<String, Todo>) {
    let title = match read_input("Title:") {
        Ok(v) => v,
        Err(e) => return println!("Error: {}", e),
    };
    let description = match read_input("Description:") {
        Ok(v) => v,
        Err(e) => return println!("Error: {}", e),
    };
    let due_date = match read_input("Due Date (dd-mm-yyyy):") {
        Ok(v) => v,
        Err(e) => return println!("Error: {}", e),
    };
    let priority = match read_input("Priority (Low, Medium, High):") {
        Ok(v) => v,
        Err(e) => return println!("Error: {}", e),
    };

    let todo = Todo {
        title: title.clone(),
        description,
        due_date,
        priority,
        status: "Pending".to_string(),
    };
    todos.insert(title, todo);
    println!("Todo added.");
}

fn read_todos(todos: &HashMap<String, Todo>) {
    for todo in todos.values() {
        println!("{:?}", todo);
    }
}

fn update_todo(todos: &mut HashMap<String, Todo>) {
    let title = match read_input("Enter the title of the todo to update:") {
        Ok(v) => v,
        Err(e) => return println!("Error: {}", e),
    };

    if let Some(todo) = todos.get_mut(&title) {
        println!("Leave field blank to keep current value.");

        if let Ok(desc) = read_input(&format!("Description [{}]:", todo.description)) {
            if !desc.is_empty() {
                todo.description = desc;
            }
        }

        if let Ok(due) = read_input(&format!("Due Date [{}]:", todo.due_date)) {
            if !due.is_empty() {
                todo.due_date = due;
            }
        }

        if let Ok(pri) = read_input(&format!("Priority [{}]:", todo.priority)) {
            if !pri.is_empty() {
                todo.priority = pri;
            }
        }

        if let Ok(stat) = read_input(&format!("Status [{}]:", todo.status)) {
            if !stat.is_empty() {
                todo.status = stat;
            }
        }

        println!("Todo updated.");
    } else {
        println!("Todo not found.");
    }
}

fn delete_todo(todos: &mut HashMap<String, Todo>) {
    let title = match read_input("Enter the title of the todo to delete:") {
        Ok(v) => v,
        Err(e) => return println!("Error: {}", e),
    };

    if todos.remove(&title).is_some() {
        println!("Todo deleted.");
    } else {
        println!("Todo not found.");
    }
}

fn save_to_file(path: &str, todos: &HashMap<String, Todo>) -> io::Result<()> {
    let mut content = String::new();
    for todo in todos.values() {
        content.push_str(&format!(
            "{}|{}|{}|{}|{}\n",
            todo.title, todo.description, todo.due_date, todo.priority, todo.status
        ));
    }
    fs::write(path, content)
}

fn load_from_file(path: &str) -> io::Result<HashMap<String, Todo>> {
    let mut todos = HashMap::new();
    if Path::new(path).exists() {
        let file = File::open(path)?;
        for line in BufReader::new(file).lines() {
            let l = line?;
            let parts: Vec<&str> = l.split('|').collect();
            if parts.len() == 5 {
                let todo = Todo {
                    title: parts[0].to_string(),
                    description: parts[1].to_string(),
                    due_date: parts[2].to_string(),
                    priority: parts[3].to_string(),
                    status: parts[4].to_string(),
                };
                todos.insert(todo.title.clone(), todo);
            }
        }
    }
    Ok(todos)
}
