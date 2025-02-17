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

fn main() {
    let mut myusers: Vec<User> = Vec::new();
    User::new(
        String::from("Alice"),
        25,
        MaritalStatus::Single,
        1,
        Sex::Female,
        &mut myusers,
    );

    if let Some(user) = myusers.get_mut(0) {
        user.updateuser_marital_status(MaritalStatus::Married);
    }

    User::new(
        String::from("Bob"),
        30,
        MaritalStatus::Married,
        2,
        Sex::Male,
        &mut myusers,
    );

    if let Some(user) = myusers.get_mut(1) {
        user.updateuser_sex(Sex::Female);
    }
    println!("{:#?}", myusers);

    if let Some(user) = myusers.get(0) {
        println!(
            "User 1's marital status: {} \n Name and id of the user is: {} and {} while his age is {}",
            user.marital_status.to_string(),  user.name, user.id, user.age
        );
    }

    if let Some(user) = myusers.get(1) {
        println!("User 2's sex: {}", user.sex.to_string());
    }

    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
        marital_status: MaritalStatus,
        id: u32,
        sex: Sex,
    }

    #[derive(Debug)]
    enum MaritalStatus {
        Single,
        Married,
    }
    impl MaritalStatus {
        fn to_string(&self) -> &'static str {
            match self {
                MaritalStatus::Single => "Single",
                MaritalStatus::Married => "Married",
            }
        }
    }

    #[derive(Debug)]
    enum Sex {
        Male,
        Female,
    }

    impl Sex {
        fn to_string(&self) -> &'static str {
            match self {
                Sex::Male => "Male",
                Sex::Female => "Female",
            }
        }
    }

    impl User {
        fn new(
            name: String,
            age: u32,
            marital_status: MaritalStatus,
            id: u32,
            sex: Sex,
            users: &mut Vec<User>,
        ) {
            let new_user = User {
                name,
                age,
                marital_status,
                id,
                sex,
            };
            users.push(new_user);
        }
        fn updateuser_marital_status(&mut self, marital_status: MaritalStatus) {
            self.marital_status = marital_status;
        }
        fn updateuser_sex(&mut self, sex: Sex) {
            self.sex = sex;
        }
    }
}
