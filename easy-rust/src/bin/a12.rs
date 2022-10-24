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
    dimensions: f32,
}

// * Use an enum for the box color
enum BoxColor {
    RED,
    BLUE,
}
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl Box {
    fn create() -> Box {
        Box {
            color: BoxColor::RED,
            weight: 1.0,
            dimensions: 2.0
        }
    }
    fn print_char(&self) {
       println!("{:?}", self.dimensions);
       println!("{:?}", self.weight);
    }
}


fn main() {
    let box_data = Box::create();
    box_data.print_char();
}
