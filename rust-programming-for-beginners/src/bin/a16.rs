// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    student_name: String,
    locker_assignment: Option<i32>,
}

fn main() {

    let my_student = Student {
        student_name: "chris".to_owned(),
        locker_assignment : None,
    };

    match my_student.locker_assignment {
        Some(assignment) => println!("Name {:?} and assignment {:?}", my_student.student_name , assignment),
        None => println!("Name {:?} and assignment None", my_student.student_name ),
    };
}


