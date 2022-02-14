#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size };
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let _rect2 = Rectangle::square(10);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
