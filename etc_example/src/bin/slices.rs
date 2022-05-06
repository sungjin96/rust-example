// Slices

fn main() {
    let seasons = ["봄", "여름", "가을", "겨울"];
    println!("{:?}", &seasons[0..2]);
    println!("{:?}", &seasons[2..]);
    println!("{:?}", &seasons[..3]);
    println!("{:?}", &seasons[..=3]);
}