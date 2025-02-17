use uuid::Uuid;
// user struct and implementation
#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub marital_status: MaritalStatus,
    pub id: Uuid,
    pub sex: Sex,
}

impl User {
    pub fn new(
        name: String,
        age: u32,
        marital_status: MaritalStatus,
        sex: Sex,
        users: &mut Vec<User>,
    ) {

        let new_user = User {
            name,
            age,
            marital_status,
            id:  Uuid::new_v4(),
            sex,
        };
        users.push(new_user);
    }
    pub fn updateuser_marital_status(&mut self, marital_status: MaritalStatus) {
        self.marital_status = marital_status;
    }
    pub fn updateuser_sex(&mut self, sex: Sex) {
        self.sex = sex;
    }
}

// marital status enums and implementation to get the string of the marital status
#[derive(Debug)]
pub enum MaritalStatus {
    Single,
    Married,
}
impl MaritalStatus {
    pub fn to_string(&self) -> &'static str {
        match self {
            MaritalStatus::Single => "Single",
            MaritalStatus::Married => "Married",
        }
    }
}

// sex status enums and implementation to get the string of the sex status
#[derive(Debug)]
pub enum Sex {
    Male,
    Female,
}

impl Sex {
    pub fn to_string(&self) -> &'static str {
        match self {
            Sex::Male => "Male",
            Sex::Female => "Female",
        }
    }
}
