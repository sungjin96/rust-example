use std::collections::HashMap;

fn main() {
    let canadian_cities = vec!["Calgray", "Vancouver"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }

    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }

    println!("{:?}", city_hashmap["Vancouver"]);
    println!("{:?}", city_hashmap.get("Vancouver"));
    println!("{:?}", city_hashmap.get("Vancoasdfasdfuver"));

    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");
    ;

    println!("{:?}", book_hashmap.get(&1));

    if let Some(book_name) = book_hashmap.get(&1) {
        println!("Already got a book: {}", book_name);
    } else {
        book_hashmap.insert(1, "Hello World");
    }
}