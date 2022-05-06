fn match_colours(rbg: (u8, u8, u8)) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, b, _) if b < 10 => println!("Not much blue"),
        (_, _, g) if g < 10 => println!("Not much green"),
        _ => println!("Every colour has at least 10")
    }
}

fn match_number(input: i32) {
    match input {
        number @ 0..=10 => println!("It's between 0 and 10. It's the number {}", number),
        _ => println!("It's greater than ten")
    }
}

fn main() {
    let first: (u8, u8, u8) = (200, 0, 0);
    let second: (u8, u8, u8) = (50, 50, 50);
    let third: (u8, u8, u8) = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);

    let my_number = 10;
    // 밑에 처럼 두가지 타입(i32, &str)를 반환하는 match는 불가능
    /*
    let some_variable = match my_number {
        10 => 8,
        _ => "Not ten"
    };
    let some_variable = if my_number == 10 { 8 } else { "Something else" };
     */

    match_number(my_number);
    match_number(100);
}