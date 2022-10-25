// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
struct Person {
    name: String,
    age: u8,
    color: Color,
}

enum Color {
    Red,
    Blue,
}
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let peoples = vec![
        Person { name: "hi".to_owned(), age: 1, color: Color::Blue }
    ];

    for person in peoples {
        if person.age <= 10 {
            print(&person.name);
        }
    }
}
