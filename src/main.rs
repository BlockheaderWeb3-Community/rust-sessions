fn main() {
    // intro_to_u();

    
    let mut new_user = User::new_user("martin".to_string(), 19, MaritalStatus::Single, SexEnum::Male, 3);
    println!("User created successfully _______ {:#?}", new_user);

    let update_marital_status = User::update_marital_status(&new_user, MaritalStatus::Married);
    println!("User update_marital_status Updated Successfully {:#?}", update_marital_status);

    // User::update_name(&mut new_user, "johnny".to_string());
    // println!("User update_name Updated Successfully {:#?}", new_user);

    let update_name = User::update_name(&new_user, "johnny".to_string());
    println!("User update_name Updated Successfully {:#?}", update_name);

    let update_age = User::update_age(&new_user, 22);
    println!("User update_name Updated Successfully {:#?}", update_age);

    let update_sex_status = User::update_sex(&new_user, SexEnum::Female);
    println!("User update_marital_status Updated Successfully {:#?}", update_sex_status);
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u8,
    marital_status: MaritalStatus,
    sex: SexEnum,
    id: u32
}

#[derive(Debug, Clone, Copy)]
enum MaritalStatus {
    Single,
    Married
}

#[derive(Debug, Copy, Clone)]
enum SexEnum {
    Male,
    Female
}

impl User {
    fn new_user(_name: String, _age: u8, _marital_status: MaritalStatus, _sex: SexEnum, _id: u32) -> User {
        let user = User {
            name: _name,
            age: _age,
            marital_status: _marital_status,
            sex: _sex,
            id: _id,
        };
        user
    }

    fn update_marital_status(&self, _marital_status: MaritalStatus) -> User {
       match _marital_status {
        MaritalStatus::Single => println!("you're single {:?}", _marital_status),
        _ => println!("you're {:?}", _marital_status),
       }

       let update_user = User {
        name: self.name.to_string(),
        age: self.age,
        marital_status: _marital_status,
        sex: self.sex.clone(),
        id: self.id
       };
       update_user
    }

    // fn update_name(&mut self, _name: String) {
    //    self.name = _name
    //  }

    fn update_name(&self, _name: String) -> User { 
        let update_user = User {
         name: _name,
         age: self.age,
         marital_status: self.marital_status.clone(),
         sex: self.sex.clone(),
         id: self.id
        };
        update_user
     }

     fn update_age(&self, _age: u8) -> User { 
        let update_user = User {
         name: self.name.to_string(),
         age: _age,
         marital_status: self.marital_status.clone(),
         sex: self.sex.clone(),
         id: self.id
        };
        update_user
     }

     fn update_sex(&self, _sex: SexEnum) -> User { 
        let update_user = User {
         name: self.name.to_string(),
         age: self.age,
         marital_status: self.marital_status.clone(),
         sex: _sex,
         id: self.id
        };
        update_user
     }

}