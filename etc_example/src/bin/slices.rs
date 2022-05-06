// Slices
// Vec

// Array, Vec
// Array = 빠르고 불편함
// Vec = 느리고 편함

// &str, String
// &str = 빠르고 불편함
// String = 느리고 편함

fn main() {
    let seasons = ["봄", "여름", "가을", "겨울"];
    println!("{:?}", &seasons[0..2]);
    println!("{:?}", &seasons[2..]);
    println!("{:?}", &seasons[..3]);
    println!("{:?}", &seasons[..=3]);
}