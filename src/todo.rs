#[derive(Debug)]
pub struct Todo {
    id: u32,
    title: String,
    status: Status,
    date: String,
    description: String,
}

#[derive(Debug)]
pub enum Status {
    Pending,
    Completed,
}

impl Todo {
    pub fn new(id: u32, title: String, status: Status, date: String, description: String) -> Todo {
        Todo {
            id: id,
            title: title,
            status: status,
            date: date,
            description: description,
        }
    }
}

// let todo_task: Vec<Todo>
