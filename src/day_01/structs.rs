#[derive(Debug)]
pub struct User {
    name: String,
    age: u32,
    user_id: u16,
    is_active: bool,
}

pub fn user_details() {
    let users = User {
        name: String::from("martin"),
        age: 45,
        user_id: 3,
        is_active: true,
    };

    let king = (10, "martin");

    let king1 = king.0;
    let king2 = king.1;

    println!("This is the user {:#?}", users);
    println!("This is the user {} {}", king1, king2);
}
