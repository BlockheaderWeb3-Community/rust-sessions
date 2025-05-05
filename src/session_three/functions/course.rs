use super::super::types::{attendance::AttendanceStruct, course::Course, student::Student};
use super::super::util::util::get_current_time;
use std::collections::HashMap;
impl Course {
    pub fn new(id: u32, name: String, capacity: u32) -> Self {
        Course {
            id,
            name,
            enrolled_students: Vec::with_capacity(capacity as usize),
            capacity,
            attendance_data: HashMap::new(),
        }
    }

    pub fn get_registered_students(&self) -> Vec<Student> {
        self.enrolled_students.to_owned()
    }

    pub fn get_registered_student_by_id(&self, student_id: usize) -> Student {
        self.enrolled_students
            .get(student_id)
            .expect("Must be a valid Student ID")
            .to_owned()
    }

    pub fn is_student_enrolled(&self, student_id: u32) -> bool {
        self.enrolled_students.iter().any(|s| s.id == student_id)
    }

    pub fn is_at_capacity(&self) -> bool {
        self.enrolled_students.len() >= self.capacity as usize
    }

    pub fn enroll_student(&mut self, student: Student) {
        self.enrolled_students.push(student);
    }

    pub fn sign_in(&mut self, student_id: u32) {
        if !self.is_student_enrolled(student_id) {
            return;
        }
        self.attendance_data
            .insert(student_id, AttendanceStruct::new());
    }

    pub fn sign_out(&mut self, student_id: u32) {
        if !self.attendance_data.contains_key(&student_id) {
            return;
        }
        let student = self.attendance_data.get_mut(&student_id).unwrap();
        student.time_out = get_current_time();
    }
}
