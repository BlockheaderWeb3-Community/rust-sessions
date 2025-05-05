use super::super::types::{course::Course, course_registry::CourseRegistry};
use super::super::util::util::convert_to_string;
use std::collections::HashMap;

impl CourseRegistry {
    pub fn new() -> CourseRegistry {
        CourseRegistry {
            courses: HashMap::new(),
            total_courses: 0,
        }
    }

    pub fn add_course(&mut self, name: &str, capacity: u32) -> u32 {
        self.total_courses += 1;
        let id = self.total_courses;
        let course = Course::new(id, convert_to_string(name), capacity);
        self.courses.insert(id, course);
        id
    }

    pub fn get_course_by_id(&self, course_id: u32) -> Course {
        self.courses
            .get(&course_id)
            .expect("Must be a valid course ID.")
            .to_owned()
    }
}
