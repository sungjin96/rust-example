// struct = and
// enum = or

//unit struct = 0 byte
struct FileDirectory;

// trait
// tuple struct
#[derive(Debug)] // attribute = println을 위한 것
struct Color(u8, u8, u8);

// named Struct
#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String
}

fn main() {
    let x = FileDirectory;
    println!("The size is {}", std::mem::size_of_val(&x));

    let my_color = Color(20, 50, 100);
    println!("The color is {:?}", my_color);

    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_string(),
        leader_name: "Justin Trudeau".to_string()
    };
    println!("The Country is: {:?}", canada);
    println!("The Country is: {:?}", canada.capital);
}
