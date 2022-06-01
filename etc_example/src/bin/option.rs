fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vew = vec![1, 2, 3, 4, 5, 6];
    let index = take_fifth(new_vew);
    println!("{:?}", index);
}