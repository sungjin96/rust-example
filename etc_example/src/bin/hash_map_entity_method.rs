use std::collections::HashMap;

fn main() {
    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "섀도우 오브 유어 스마일",
        "eye of the World",
        "eye of the World",
    ];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let number_of_books = book_hashmap.entry(book).or_insert(0);
        *number_of_books += 1;
    }

    for (book, number) in book_hashmap {
        println!("Do we have {}? {}", book, number);
    }
}