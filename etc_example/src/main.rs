fn main() {
}
// ==============================
// 17 강
// Ownership - 소유권
// & = reference
// 밑 함수 처럼 진행하면 안됨
// 이유 - country 변수가 언제 어떻게 사라지고 사용될지 알 수 없음.
// 메모리 보안?안전을 위한 이유임
fn return_it() -> &String {
    let country = String::from("대한민국");
    &country
}

fn returning_a_reference() {
    println!("Country is: {}", return_it());
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

fn _const_and_static() {
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
