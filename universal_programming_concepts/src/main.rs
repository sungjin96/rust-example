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
    println!("The value of spaces is: {}", spaces);

    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let b: (i32, f64, u8) = (500, 6.5, 1);
    let _five_hundred = b.0;
    let _size_point_four = b.1;
    let _one = b.2;
    let _arr = [1, 2, 3, 4, 5];
    let _months = ["January", "February"];

}
