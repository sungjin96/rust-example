// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
struct Box {
    color: BoxColor,
    weight: f32,
    dimensions: Dimensions,
}

// * Use an enum for the box color
enum BoxColor {
    RED,
    BLUE,
}

impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::RED => println!("레드!"),
            BoxColor::BLUE => println!("파랑!"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl Box {
    fn create() -> Box {
        Box {
            color: BoxColor::RED,
            weight: 1.0,
            dimensions: Dimensions {
                width: 1.0,
                height: 1.0,
                depth: 1.0,
            },
        }
    }
    fn print_char(&self) {
        self.color.print();
        println!("{:?}", self.weight);
    }
}


fn main() {
    let box_data = Box::create();
    box_data.print_char();
}
