fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
    println!("The value of five is: {}", five());
    println!("The value of plus_one is: {}", plus_one(5));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {} in function", x);
    println!("The value of y is: {} in function", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
