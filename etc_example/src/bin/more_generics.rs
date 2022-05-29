// 프린트 할때 필요
use std::fmt::Display;

// 비교할 때
use std::cmp::PartialOrd;

fn compare_and_print<T, U>(statement: T, num_1: U, num_2: U)
    where
        T: Display,
        U: Display + PartialOrd
{
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    )
}

fn main() {
    compare_and_print("Listen up!", 9, 8);
}