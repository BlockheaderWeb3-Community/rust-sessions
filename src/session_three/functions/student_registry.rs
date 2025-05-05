use super::super::types::{
    course::Course, course_registry::CourseRegistry, sex::Sex, student::Student,
    student_registry::StudentRegistry,
};

impl StudentRegistry {
    // this initializes a new StudentRegistry
    // new array of students
    pub fn new_session() -> StudentRegistry {
        StudentRegistry {
            total_students: Vec::new(),
            course_registry: CourseRegistry::new(),
        }
    }

    // register student
    pub fn register(
        &mut self,
        first_name: String,
        last_name: String,
        age: u8,
        height: f32,
        sex: Sex,
    ) -> Student {
        // generate a unique ID
        let student_id: u32 = self.total_students.len() as u32 + 1;
        let student = Student::new(age, height, student_id, first_name, last_name, sex);
        self.total_students.push(student.clone());
        student
        // student_id
    }

    // util function to get student by id
    pub fn get_student_by_id(&self, id: u32) -> Option<&Student> {
        self.total_students.get((id - 1) as usize)
    }

    pub fn enroll_student_course(&mut self, course_id: u32, student_id: u32) -> Result<(), String> {
        // Validate course and student exists in the registry,
        // Return a mutable borrow if they do.
        let (course, student) = self.get_course_and_student(course_id, student_id)?;

        // Check if student is already enrolled for this course,
        // No double enrolls allowed.
        if course.is_student_enrolled(student.id) {
            return Err(format!(
                "Student with ID {} is already enrolled in course {:#?}",
                student_id, course.name
            ));
        }

        // Confirm students enrolled in the course are within the class size.
        // Prevent enrollment if otherwise.
        if course.is_at_capacity() {
            return Err(format!(
                "Class capacity exceeded: {}/{} students enrolled.",
                course.enrolled_students.len(),
                course.capacity
            ));
        }

        // Enroll student in course
        course.enroll_student(student.to_owned());
        student.enroll_in_course(course_id, course.name.to_owned());

        Ok(())
    }

    pub fn get_course_and_student(
        &mut self,
        course_id: u32,
        student_id: u32,
    ) -> Result<(&mut Course, &mut Student), String> {
        let course = self
            .course_registry
            .courses
            .get_mut(&course_id)
            .ok_or_else(|| {
                format!(
                    "Invalid Course ID. There are {} courses available.",
                    self.course_registry.total_courses
                )
            })?;
        let student = self
            .total_students
            .get_mut((student_id - 1) as usize)
            .ok_or_else(|| format!("Invalid Student ID: {}", student_id))?;

        Ok((course, student))
    }
}
