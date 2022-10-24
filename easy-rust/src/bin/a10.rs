// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
fn println_message(gt_100: bool) {
    match gt_100 {
        true => println!("its big"),
        false => println!("its small")
    }
}
// * Use a match expression to determine which message
//   to print

fn main() {
    let num = 100;
    let result = num > 100;
    println_message(result);
}
