// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
struct Grocery {
    id: i32,
    quantity: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(grocery: &Grocery) {
   println!("{:?}", grocery.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id(grocery: &Grocery) {
    println!("{:?}", grocery.id);
}

fn main() {
    let grocery = Grocery {
        id: 1,
        quantity: 10
    };

    display_quantity(&grocery);
    display_id(&grocery);
}
