// use uuid::Uuid as;

#[derive(Debug)]
struct User {
    // id: Uuid,
    id: u128,
    name: String,
    gender: Sex,
    marital_status: Status,
}

#[derive(Debug)]
enum Status {
    Married,
    Single,
    Divorce,
    Widow,
}

#[derive(Debug)]
enum Sex {
    Male,
    Female,
}

// util function to convert from u128
fn convert_usize_to_u128(len: usize) -> u128 {
    len as u128
}

impl User {
    fn new_user(users: &Vec<User>, name: String, gender: Sex, marital_status: Status) -> User {
        User {
            id: convert_usize_to_u128(users.len()) + 1,
            name,
            gender,
            marital_status,
        }
    }

    fn update_user_name(name: String, users_data: &mut Vec<User>, id: u128) {
        let updated_name = users_data.iter_mut().find(|x| x.id == id).unwrap();
        updated_name.name = name;
    }
    fn update_user_sex(gender: Sex, users_data: &mut Vec<User>, id: u128) {
        let updated_sex = users_data.iter_mut().find(|x| x.id == id).unwrap();
        updated_sex.gender = gender;
    }
    fn update_user_marital_status(status: Status, users_data: &mut Vec<User>, id: u128) {
        let updated_status = users_data
            .iter_mut()
            .find(|x: &&mut User| x.id == id)
            .unwrap();
        updated_status.marital_status = status;
    }

    // fn find_user(|x: &&mut User)
}

fn main() {
    let mut users_data: Vec<User> = Vec::new();
    println!("new users here: {:#?}", users_data);
    users_data.push(User::new_user(
        &users_data,
        "yunus".to_string(),
        Sex::Male,
        Status::Single,
    ));
    println!("new user 1 here: {:#?}", users_data);

    users_data.push(User::new_user(
        &users_data,
        "Titilola".to_string(),
        Sex::Female,
        Status::Divorce,
    ));
    // users_data.push(User::new_user(
    //     "yunus".to_string(),
    //     Sex::Male,
    //     Status::Married,
    // ));
    // users_data.push(User::new_user(
    //     "funke".to_string(),
    //     Sex::Female,
    //     Status::Widow,
    // ));

    // users_data.push(User::new_user(
    //     "funke".to_string(),
    //     Sex::Female,
    //     Status::Widow,
    // ));

    User::update_user_name("Iliya".to_string(), &mut users_data, 2);

    // User::update_user_marital_status(Status::Married, &mut users_data);
    // User::update_user_sex(Sex::Female, &mut users_data, 3);
    // User::update_user_name("kemi".to_string(), &mut users_data, 2);
    println!("updated user here: {:#?}", users_data);

    User::update_user_name("Martins".to_string(), &mut users_data, 21);

    println!("updated user 3 here: {:#?}", users_data);
}
