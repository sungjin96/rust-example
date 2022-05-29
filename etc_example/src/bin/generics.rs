use std::fmt::Display;

fn print_and_give_item() -> i32 {
    let number = 9;
    println!("The number is: {}", number);
    number
}

fn give_thing<T: Display>(input: T) -> T {
    println!("{}", input);
    input
}


fn main() {
    let x = print_and_give_item();
    let z = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
}
