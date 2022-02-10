fn main() {
    // is Immutable ( 변수 )
    // 참고 - 변수지만 Immutable 함. 상수(const)는 말 그대로 상수, Immutable 변수는 변수의 기본 설정이 Immutable.
    // 즉 상수랑은 다름.
    // let x = 5;

    // is Mutable ( 변수 )
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let a = 5;
    let a = a + 1;
    let a = a * 2;
    println!("The value of a is: {}", a);

    let spaces = "    ";
    let spaces = spaces.len();
}
