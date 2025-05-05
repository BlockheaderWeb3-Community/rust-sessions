use super::course_registry::CourseRegistry;
use super::student::Student;
// student registry struct
/// This struct holds the student information and the course registry
/// It contains a vector of Student structs and a CourseRegistry struct
#[derive(Debug, Clone)]
pub struct StudentRegistry {
    pub total_students: Vec<Student>,
    pub course_registry: CourseRegistry,
}
