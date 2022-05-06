// struct = and
// enum = or

// unit struct = 아무것도 없는 struct, 0 byte
struct FileDirectory;

// tuple struct
#[derive(Debug)]
struct Colour(u8, u8, u8);

// name struct
#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn takes_file_directory(input: FileDirectory) {
    println!("I got a file directory");
}

fn main() {
    let x = FileDirectory;
    println!("{}", std::mem::size_of_val(&x));
    takes_file_directory(x);

    let my_colour = Colour(20, 50, 100);
    println!("{:?}", my_colour);
    println!("The second colour is {}", my_colour.1);

    let kr = Country {
        population: 1000,
        capital: String::from("A"),
        leader_name: String::from("V"),
    };
    println!("{:#?}", kr);
    println!("The Population is: {}", kr.population);
    println!("The Capital is: {}", kr.capital);
    println!("The LeaderName is: {}", kr.leader_name);
}