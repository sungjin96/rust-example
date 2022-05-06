fn main() {
    let my_number = 7;
    if my_number == 7 {
        println!("It's seven");
    } else if my_number == 6 {
        println!("It's six");
    } else {
        println!("It's a different number");
    }

    let second_number = match my_number {
        0 => 23,
        1 => 53,
        2 => 102,
        _ => 0
    };
    println!("{second_number}")
}