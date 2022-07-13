use std::collections::VecDeque;

fn main() {
    let mut my_vec = VecDeque::from(vec![0; 600_000]);
    for _i in 0..600000 {
        my_vec.pop_front();
    }
}