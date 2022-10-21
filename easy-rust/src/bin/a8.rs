// Topic: Organizing similar data using structs //
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Sparkling,
    Sweet,
    Fruity
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_run(drink: Drink) {

    match drink.flavor {
        Flavor::Fruity => println!("Fruity"),
        Flavor::Sweet => println!("Sweet"),
        Flavor::Sparkling => println!("Sparkling")
    }

    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 1.2
    };

    print_run(drink);
}
