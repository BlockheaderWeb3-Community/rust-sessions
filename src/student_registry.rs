use std::collections::HashMap;
struct StudentData {
    data:HashMap<String,StudentRegistry>
}

#[derive(Debug,Clone)]
struct StudentRegistry{
    id: u32,
    name: String,
    status:Status,
    gender:Gender,
    year:u8,
    age:u8,
    department:Department,
    email:String
}


#[derive(Clone,Debug,Copy)]
enum Department{
    MassCom,
    ComputerScience,
    Medicine,
    Law
}

#[derive(Clone,Debug,Copy)]
enum Gender {
    Male,
    Female
}

#[derive(Clone,Debug,Copy)]
enum Status {
    Registered,
    Expelled,
    Suspended,
    Graduated
}

impl StudentData  {
    fn register_student(&mut self, name:String, age: u8, department:Department, gender: Gender, email:String)->Result<String,String>{
        let length : u32= (self.data.len() + 1) as _;
        let is_student_exist = self.data.contains_key(&email);
        
        if !is_student_exist {
            let student_data = StudentRegistry{
            age,
            department,
            gender,
            id: length,
            name,
            status:Status::Registered,
            year:1,
            email:email.clone()
        };
        
        self.data.insert(email, student_data);
        Ok("successfully registered".to_string())
        }
        else {
            Err("user already exist".to_string())
        }

    }

    fn get_all_students(self)->HashMap<String,StudentRegistry>{
        self.data
    }

    fn get_student_by_id(self,id:String)->Result<StudentRegistry,String>{
        let student = self.data.get(&id);
        match student {
            Some(data) => {
                Ok(data.clone())
            }
            None=>{
                Err("user does not exist".to_string())
            }
        }
    }


}
