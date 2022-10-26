// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

#[derive(Debug)]
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student = Student {
        name: "Hello".to_owned(),
        locker: Some(3),
    };

    println!("{:?}", student);
    match student.locker {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("no locker assigned")
    }
}
