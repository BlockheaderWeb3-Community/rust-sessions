
// pub mod arithmetic_ops;
// pub mod check_func;
// pub mod string_ops;
// pub mod calculator;

// mod float;
// mod signed;
// mod string;
// mod unsigned;

// fn main() {
//     // intro_to_rust::intro_to_rust();
//     // intro_to_rust::print_arithmetic_ops_results();
//     // intro_to_rust::print_string_ops_results();
//     // intro_to_rust::check_func_results();
  
// //       unsigned::intro_to_u();
// //     signed::intro_to_i();
// //     float::intro_to_float();
// //     string::strings();
  
//     calculator::main();
// }
// pub mod intro_to_rust {
//     use crate::{
//         arithmetic_ops::{
//             arithmetic_floatingpoint_ops, arithmetic_signed_ops, arithmetic_unsigned_ops,
//             integer_type_casting,
//         },
//         check_func::check_func,
//         string_ops::{
//             intro_to_str_slice, string_conversion_ops, string_formatting, string_handler,
//         },
//     };

//     pub fn intro_to_rust() {
//         println!("Welcome to Rust Programming Language");
//     }

//     pub fn print_arithmetic_ops_results() {
//         println!("The Sum, substraction, multiplication and division of the two numbers passed in this function are   {:?}", arithmetic_signed_ops(20, 3));
//         println!(
//             "This function types casts the first u32 to u8 amd the second u8 to u32 {:?} ",
//             integer_type_casting(20, 3)
//         );
//         println!("The Sum, substraction, multiplication and division of the two numbers passed in this function are    {:?}", arithmetic_unsigned_ops(20, 3));
//         println!("The Sum, substraction, multiplication and division of the two numbers passed in this function are    {:?}", arithmetic_floatingpoint_ops(2.0, 3.0));
//     }

//     pub fn print_string_ops_results() {
//         let first_name = String::from("AKinshola");
//         let last_name = String::from("Akinniyi");
//         println!(
//             "The full name of the person is {}",
//             string_formatting(first_name, last_name)
//         );
//         println!(
//             "The string literal and the main string are {:?}",
//             string_conversion_ops("Hello", &String::from("World"))
//         );
//         println!("The string slice is {:?}", intro_to_str_slice());
//         string_handler();
//     }
//     pub fn check_func_results() {
//         println!(" {:?} ", check_func(8, 10));
//     }
// }

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
        &mut myusers
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
        &mut myusers
    );

    if let Some(user) = myusers.get_mut(1) {
        user.updateuser_sex(Sex::Female);
    }
    println!("{:?}", myusers);

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
        fn new(name: String, age: u32, marital_status: MaritalStatus, id: u32, sex: Sex, users: &mut Vec<User>) {
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
        fn updateuser_sex(&mut self, sex: Sex,){
            self.sex = sex;
        }
    }
}

