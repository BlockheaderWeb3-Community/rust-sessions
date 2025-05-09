// mod functions;
// mod types;
// mod util;
use super::types::{sex::Sex, student_registry::StudentRegistry};

pub fn student_registry() {
    let mut student_registry = StudentRegistry::new_session();
    let course_id = student_registry.course_registry.add_course("Math", 30);
    let student = student_registry.register(
        "John".to_string(),
        "Doe".to_string(),
        20,        // age
        5.9,       // height
        Sex::Male, // sex
    );

    let student_2 = student_registry.register(
        "kate".to_string(),
        "samuel".to_string(),
        22,  // age
        5.7, // height
        Sex::Female,
    );
    student_registry
        .enroll_student_course(course_id, student.id)
        .expect("Failed to enroll student in course");
    student_registry
        .enroll_student_course(course_id, student_2.id)
        .expect("Failed to enroll student in course");
    let course = student_registry.course_registry.get_course_by_id(course_id);
    println!("Course ID: {:#?}", course);
    // println!("Course Name: {}", course.name);
    // println!("Enrolled Students: {:?}", course.get_registered_students());
    // println!("Student ID: {}", student.id);
    // println!("Student Name: {} {}", student.first_name, student.last_name);
    // println!("Student Age: {}", student.age);

    // create course 1 with max capacity of 1 student
    let course_1 = student_registry
        .course_registry
        .add_course("Introduction To Rust", 1);

    // create course 2 with max capacity of 3 students
    let course_2 = student_registry
        .course_registry
        .add_course("Advanced Rust", 3);

    println!("course 1 is created with id: {course_1:#?}");
    println!("course 2 is created with id: {course_2:#?}");
    print!("\n\n");

    // attempt to register john for course 1
    // john student id is 0
    let course_1_reg_result_1 = student_registry.enroll_student_course(course_1, 1);

    // attempt to register rico for course 1
    // kate student id is 0
    // registration should return an error because course 1 has max capacity of 1 student
    let course_1_reg_result_2 = student_registry.enroll_student_course(course_1, 2);

    println!("john registration result for course_1: {course_1_reg_result_1:#?}");
    println!("kate registration result for course_2: {course_1_reg_result_2:#?}");
    print!("\n\n");

    // attempt to register john for course 1
    // john student id is 0
    // also test that student can register for more than 1 course
    let course_2_reg_result_1 = student_registry.enroll_student_course(course_2, 1);

    // attempt to register stephanie for course 1 twice
    // jonh student id is 0
    // should fail as double registration are not allowed
    let course_2_reg_result_2 = student_registry.enroll_student_course(course_2, 1);

    // attempt to register rico for course 1
    // rico student id is 0
    // should return an Ok result since max capacity is not reached
    let course_2_reg_result_3 = student_registry.enroll_student_course(course_2, 2);

    println!("john registration result for course_2: {course_2_reg_result_1:#?}");
    println!("john second registration result for course_2: {course_2_reg_result_2:#?}");
    println!("kate registration result for course_2: {course_2_reg_result_3:#?}");
}

mod farm {
    pub fn identifier() {
        println!("Farm identifier");
    }
}
