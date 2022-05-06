// Vec

// Array, Vec
// Array = 빠르고 불편함
// Vec = 느리고 편함

// &str, String
// &str = 빠르고 불편함
// String = 느리고 편함

fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let my_vec = vec![name1, name2];
    println!("{}", my_vec.capacity());
    println!("{:?}", my_vec);
}