use std::collections::HashMap;

pub struct StudentData {
    data: HashMap<String, StudentRegistry>,
}

#[derive(Debug, Clone)]
struct StudentRegistry {
    pub id: u32,
    pub name: String,
    pub status: Status,
    pub gender: Gender,
    pub year: u8,
    pub age: u8,
    pub department: Department,
    pub email: String,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Department {
    MassCom,
    ComputerScience,
    Medicine,
    Law,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    Registered,
    Expelled,
    Suspended,
    Graduated,
}

impl StudentData {
    fn register_student(
        &mut self,
        name: String,
        age: u8,
        department: Department,
        gender: Gender,
        email: String,
    ) -> Result<String, String> {
        let length: u32 = (self.data.len() + 1) as _;
        let is_student_exist = self.data.contains_key(&email);

        if !is_student_exist {
            let student_data = StudentRegistry {
                age,
                department,
                gender,
                id: length,
                name,
                status: Status::Registered,
                year: 1,
                email: email.clone(),
            };

            self.data.insert(email, student_data);
            Ok("successfully registered".to_string())
        } else {
            Err("user already exist".to_string())
        }
    }

    fn get_all_students(self) -> HashMap<String, StudentRegistry> {
        self.data
    }

    fn get_student_by_id(self, id: String) -> Result<StudentRegistry, String> {
        let student = self.data.get(&id);
        match student {
            Some(data) => Ok(data.clone()),
            None => Err("user does not exist".to_string()),
        }
    }

    fn get_student_by_status(self, status: Status) -> Vec<StudentRegistry> {
        let student_collection = self.data;
        let mut student_data = Vec::new();

        for (_, value) in student_collection {
            if value.status == status {
                student_data.push(value)
            }
        }

        student_data
    }

    fn change_status(mut self, id: String, status: Status) -> Result<String, String> {
        let student = self.data.contains_key(&id);

        if student {
            let current_student = self.data.get(&id).unwrap();
            let updated_student_data = StudentRegistry {
                status: status,
                ..current_student.clone()
            };
            self.data.insert(id, updated_student_data);
            Ok("successfully update".to_string())
        } else {
            Err("failed to update".to_string())
        }
    }

    fn get_by_gender(self, gender: Gender) -> Vec<StudentRegistry> {
        let student_collection = self.data;
        let mut student_data = Vec::new();

        for (_, value) in student_collection {
            if value.gender == gender {
                student_data.push(value)
            }
        }

        student_data
    }

    fn get_student_by_department(self, department: Department) -> Vec<StudentRegistry> {
        let student_collection = self.data;
        let mut student_data = Vec::new();

        for (_, value) in student_collection {
            if value.department == department {
                student_data.push(value)
            }
        }

        student_data
    }
}
