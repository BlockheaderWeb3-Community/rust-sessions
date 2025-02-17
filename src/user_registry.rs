#[derive(Clone, Debug)]
pub struct User {
    id: i8,
    name: String,
    age: i16,
    status: MarriedStatus,
    gender: Sex,
}
#[derive(Clone, Copy, Debug)]
pub enum MarriedStatus {
    Single,
    Married,
    Divorced,
}

#[derive(Clone, Copy, Debug)]
pub enum Sex {
    Male,
    Female,

}

    impl User {
        pub fn users(_id: i8, _name: String , _age:i16, _status: MarriedStatus, _gender: Sex) -> User {
            let user = User {
                id: _id,
                name: _name,
                age: _age,
                status: _status,
                gender: _gender,
            };
            user
        }

        pub fn update_marriage_status(&mut self, status: MarriedStatus) -> User {
            match status {
                MarriedStatus::Married => {
                    println!("The user is : {:?}", status);
                }
                MarriedStatus::Single => {
                    println!("The user is: {:?}", status);
                }
                MarriedStatus::Divorced => {
                    println!("The user is: {:?}", status);
                }
            
            }
            let user = User {
                id: self.id,
                name: self.name.to_string(),
                age: self.age,
                status: status,
                gender: self.gender,
            };
            user
            
        }

        pub fn update_the_gender(&self, gender: Sex) -> User {
            match gender {
                Sex::Male => {
                    println!("The user is : {:?}", gender);
                }
                Sex::Female => {
                    println!("The user is : {:?}", gender);
                }
                
                
            }
            let user = User {
                id: self.id,
                name: self.name.clone(),
                age: self.age,
                status: self.status.clone(),
                gender: self.gender,

            };
            user

    }

        pub fn update_user_name(&mut self, _name: String) {
                self.name = _name.to_string(); 
            }
         
 
    }
    

    


    
pub fn user_reg() {
    
    // let all_users = [

    //     User {id: 0, name: "David".to_string(), age: 20, status: MarriedStatus::Single, gender: Sex::Male},
    //     User {id: 1, name: "Oshioke".to_string(), age: 40, status: MarriedStatus::Divorced, gender: Sex::Female},
    //     User {id: 2, name: "Ibukun".to_string(), age: 24, status: MarriedStatus::Married, gender: Sex::Female},
    //     User {id: 3, name: "Andrew".to_string(), age: 26, status: MarriedStatus::Divorced, gender: Sex::Male},
    //     User {id: 4, name: "Emmanuel".to_string(), age: 34, status: MarriedStatus::Single, gender: Sex::Male},
    // ];

    // match all_users {
    //     [User { id: 0, name, .. }, ..] => println!("First user is {}", name),
    //     [_, User { id: 1, name, .. }, ..] => println!("Second user is {}", name),
    //     _ => println!("User is not Registered"),
    // }

    let mut all_users = vec![
        User {id: 0, name: "David".to_string(), age: 20, status: MarriedStatus::Single, gender: Sex::Male},
        User {id: 1, name: "Oshioke".to_string(), age: 40, status: MarriedStatus::Divorced, gender: Sex::Female},
        User {id: 2, name: "Ibukun".to_string(), age: 24, status: MarriedStatus::Married, gender: Sex::Female},
        User {id: 3, name: "Andrew".to_string(), age: 26, status: MarriedStatus::Divorced, gender: Sex::Male},
        User {id: 4, name: "Emmanuel".to_string(), age: 34, status: MarriedStatus::Single, gender: Sex::Male},
        ];
        for user in &mut all_users {
            println!("all users in here are: {:?}", user);
        }


    

    let mut new_user = User::users( 0,"David".to_string(),  28,  MarriedStatus::Married, Sex::Male);
    println!("The user here: {:?} ", new_user);


    let update_status  = User::update_marriage_status( &mut new_user,  MarriedStatus::Single);  
    println!("The updated status: {:?}", update_status);

    let update_gender: User = User::update_the_gender(&mut new_user, Sex::Female);
    println!("The update gender: {:?}", update_gender);

    let update_name = User::update_user_name(&mut new_user, "Caleb".to_string());
    println!("The updated username is: {:?}", update_name); 

    }
