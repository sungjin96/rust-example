enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    }
}

fn main() {
    use Star::*;
    let star_vec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];

    for star in star_vec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star: {}", size),
            size if size >= 80 => println!("Pretty big star: {}", size),
            _ => println!("Some other star")
        }
    }

    println!("What about DeadStart? it is: {}", DeadStar as u32);

    let my_vec = vec![get_number(-300), get_number(8)];
    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {}", number),
            Number::I32(number) => println!("It's a i32 with the value {}", number),
        }
    }
}