// //mod strings;
// mod float;
// mod signed;
// mod string;
// mod unsigned;
// fn main() {
//     unsigned::intro_to_u();
//     signed::intro_to_i();
//     float::intro_to_float();
//     string::strings();
// }

mod types;
use crate::types::*;
fn main() {
    let mut myusers: Vec<User> = Vec::new();

    // create new user
    User::new(
        String::from("John"),
        25,
        MaritalStatus::Single,
        1,
        Sex::Female,
        &mut myusers,
    );

    // create user 2
    User::new(
        String::from("Akin"),
        30,
        MaritalStatus::Married,
        2,
        Sex::Male,
        &mut myusers,
    );

    // update first user marital status
    if let Some(user) = myusers.get_mut(0) {
        user.updateuser_marital_status(MaritalStatus::Married);
    }

    // update second user sex
    if let Some(user) = myusers.get_mut(1) {
        user.updateuser_sex(Sex::Female);
    }

    // prints user vec
    println!("This is the whole user vec data \n {:#?}", myusers);

    // printing all data from user 1
    print!("--- Printing the data from the user Vec --- \n");
    if let Some(user) = myusers.get(0) {
        println!(
            "User 1's marital status: {} \n Name and id of the user is: {} and {} while his age is {}",
            user.marital_status.to_string(),  user.name, user.id, user.age
        );
    }

    // printing all data from user 1
    if let Some(user) = myusers.get(1) {
        println!(
            "User 2's sex: {} \n Name and id of the user is: {} and {} while his age is {}",
            user.sex.to_string(),
            user.name,
            user.id,
            user.age
        );
    }
}
