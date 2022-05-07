use std::mem::size_of_val;

#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

struct Number {
    one: u8,
    two: u8,
    three: u8,
}

fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justin Trudeau".to_string();

    let my_country = Country { population, capital, leader_name };

    println!("{:#?}", my_country);
    println!("{:#?}", size_of_val(&my_country));

    let numbers = Number {
        one: 8,
        two: 18,
        three: 20
    };

    println!("Size is: {}", size_of_val(&numbers));
}