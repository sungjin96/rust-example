fn match_number(input: i32) {
    match input {
        number @ 0..=10 => println!("it's between 0 and 10. {}", number),
        _ => println!("It's greater than ten")
    }
}

fn main() {

}
