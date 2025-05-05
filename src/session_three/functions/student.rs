use super::super::types::{sex::Sex, student::Student};
use std::collections::HashMap;

impl Student {
    pub fn new(
        age: u8,
        height: f32,
        id: u32,
        first_name: String,
        last_name: String,
        sex: Sex,
    ) -> Self {
        Student {
            age,
            first_name,
            last_name,
            id,
            height,
            sex,
            enrolled_courses: HashMap::new(),
        }
    }

    pub fn get_registered_courses(&self) -> HashMap<u32, String> {
        self.enrolled_courses.to_owned()
    }

    pub fn get_registered_course_by_id(&self, course_id: u32) -> String {
        self.enrolled_courses
            .get(&course_id)
            .expect("Must be a valid Course ID.")
            .to_owned()
    }

    pub fn enroll_in_course(&mut self, course_id: u32, course_name: String) {
        self.enrolled_courses.insert(course_id, course_name);
    }
}
