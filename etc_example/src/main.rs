use std::mem::size_of_val;

#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

struct Numbers {
    one: u8,
    two: u8,
    three: u8,
    four: char
}


fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justin Trudeau".to_string();
    let canada = Country { population, capital, leader_name };
    println!("The Country is: {:#?}", canada);
    println!("The Country is: {:?}", canada.capital);
    println!("Country is {} bytes in size", size_of_val(&canada));

    let numbers = Numbers {
        one: 8,
        two: 19,
        three: 20,
        four: 'a'
    };

    println!("Numbers is {} bytes in size", size_of_val(&numbers));
}
