#[derive(Debug)]
pub struct User {
    name: String,
    age: u32,
    user_id: u16,
    is_active: bool
}

#[allow(dead_code)]
pub fn user_details() {
    let user = User {
        name: String::from("Edoh"),
        age: 22,
        user_id: 1,
        is_active: true
    };    

    println!("User {:?}", user);
    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("User ID: {}", user.user_id);
    println!("Is Active: {}", user.is_active);
}
