#![allow(warnings)]

use std::cell::RefCell;
use std::rc::Rc;

// struct Student<'a> {
//     name: String,
//     courses: Vec<&'a Course<'a>>,
// }

// impl<'a> Student<'a> {
//     fn new(name: &str) -> Student<'a> {
//         Student {
//             name: name.into(),
//             courses: Vec::new(),
//         }
//     }
// }

// struct Course<'a> {
//     name: String,
//     students: Vec<&'a Student<'a>>,
// }

// impl<'a> Course<'a> {
//     fn new(name: &str) -> Course<'a> {
//         Course {
//             name: name.into(),
//             students: Vec::new(),
//         }
//     }

//     fn add_student(&'a mut self, student: &'a mut Student<'a>) {
//         student.courses.push(self); // immutable borrow occurs here
//         self.students.push(student); // cannot borrow `self.students` as mutable because it is also borrowed as immutable
//     }
// }

// struct Student {
//     name: String,
//     courses: Vec<Rc<RefCell<Course>>>,
// }

// impl Student {
//     fn new(name: &str) -> Student {
//         Student {
//             name: name.into(),
//             courses: Vec::new(),
//         }
//     }
// }

// struct Course {
//     name: String,
//     students: Vec<Rc<RefCell<Student>>>,
// }

// impl Course {
//     fn new(name: &str) -> Course {
//         Course {
//             name: name.into(),
//             students: Vec::new(),
//         }
//     }

//     fn add_student(course: Rc<RefCell<Course>>, student: Rc<RefCell<Student>>) {
//         student.borrow_mut().courses.push(course.clone());
//         course.borrow_mut().students.push(student.clone());
//     }
// }

// fn circular_references() {
//     let student = Rc::new(RefCell::new(Student::new("student")));
//     let course = Course::new("course");
//     let magic_course = Rc::new(RefCell::new(course));

//     Course::add_student(magic_course.clone(), student);

//     let student2 = Rc::new(RefCell::new(Student::new("student2")));
//     Course::add_student(magic_course.clone(), student2);
// }

struct Student {
    name: String,
}

impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform
            .enrollments
            .iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

struct Course {
    name: String,
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course,
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>,
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new(),
        }
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course));
    }
}

fn circular_references() {
    let s = Student {
        name: "Student".into(),
    };
    let c = Course {
        name: "Course".into(),
    };

    let mut p = Platform::new();
    p.enroll(&s, &c);

    for c in s.courses(p) {
        println!("s has taken course: {}", c)
    }
}

fn main() {
    circular_references();
}
