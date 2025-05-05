use super::course::Course;
use std::collections::HashMap;
/// course registery struct
/// This struct holds the course information and the total number of courses
/// It contains a HashMap where the key is the course ID and the value is a Course struct
#[derive(Debug, Clone)]
pub struct CourseRegistry {
    pub courses: HashMap<u32, Course>,
    pub total_courses: u32,
}
