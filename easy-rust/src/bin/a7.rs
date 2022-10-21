// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a Color to the terminal
//
// Notes:
// * Use an enum with Color names as variants
// * Use a function to print the Color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which Color
//   name to print

enum Color {
    Red,
    Yellow,
    Blue
}

fn print_color(color: Color) {

    match color {
        Color::Red => println!("the red"),
        Color::Yellow => println!("the yellow"),
        Color::Blue => println!("the Blue"),
    }
}

fn main() {
    let color = Color::Blue;
    print_color(color);
}
