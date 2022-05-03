fn main() {
    const_and_static();
}
// ==============================


// 16 강
// const
// static
//
// 1. Type declaration required
// 2. Only Uppercase name ( ex - NUMBER, HIGH_SCORE ... )
const HIGH_SCORE: i32 = 10;
static LOW_SCORE: i32 = 0; // unsafe 로 수정은 가능하지만 다른 방법을 고민 해야함.

fn print_high_score() {
   println!("The high score is {HIGH_SCORE}") ;
}

fn const_and_static() {
   print_high_score();
}
// ==============================

// 15 강
// capacity / push / push_str / pop / with_capacity
fn _string_method() {
    let mut my_name = String::with_capacity(4);
    println!("{}", my_name.capacity());
    my_name.push_str("SJ");
    println!("{}", my_name.capacity());
    my_name.push('!');
    println!("My name is {}", my_name);
}
// ==============================

// 14 강
fn _string_and_str() {
    // String = Sized type -> heap
    // &str = Dynamic type
    let _my_name = "SJ"; // &str
    let _other_name = String::from("SJ");

    let mut my_other_name = "SJ".to_string();
    my_other_name.push('!');
    println!("{my_other_name}");
}
// ==============================
