// & immutable reference / shared reference
// &mut mutable reference / unique reference

fn main() {
    // ERROR !!!
    // let mut number = 10;
    // let number_ref = &number;
    // let number_change = &mut number;
    // *number_change += 10;
    // println!("{}", number_ref);

    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10;
    let number_ref = &number;
    println!("{}", number_ref);

    let country = "대한민국";
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country);
}
