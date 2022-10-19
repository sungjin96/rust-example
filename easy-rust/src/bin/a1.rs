// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
fn first_name() {
   println!("Jang");
}
// * Use a function to display your last name
fn last_name() {
   println!("Sungjin");
}
// * Use the println macro to display messages to the terminal

fn main() {
    display("장", "성진")
}

fn display(first: &str, last: &str) {
    println!("{:?} {:?}", first, last);
}
