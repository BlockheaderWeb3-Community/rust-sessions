use super::super::types::sex::Sex;
use std::collections::HashMap;

// student struct
/// This struct holds the student information
#[derive(Debug, Clone)]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
    pub id: u32,
    pub age: u8,
    pub height: f32,
    pub sex: Sex,
    pub enrolled_courses: HashMap<u32, String>,
}
