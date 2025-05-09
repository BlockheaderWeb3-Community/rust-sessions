use super::attendance::AttendanceStruct;
use super::student::Student;
use std::collections::HashMap;

// course struct
/// This struct holds the course information
/// It contains the course ID, name, capacity, enrolled students, and attendance data
/// The enrolled students are stored in a vector of Student structs
/// The attendance data is stored in a HashMap where the key is the student ID and the value is an AttendanceStruct
/// The attendance data contains the date, time in, time out, and attendance status
/// The attendance status is an enum that can be either Present or Absent
/// The course ID is a unique identifier for each course
#[derive(Debug, Clone)]
pub struct Course {
    pub id: u32,
    pub name: String,
    pub capacity: u32,
    pub enrolled_students: Vec<Student>,
    pub attendance_data: HashMap<u32, AttendanceStruct>,
}
