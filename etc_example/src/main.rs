// Ownership and copy types
// copy - copy types
// clone - non-copy types

fn prints_number(number: i32) {
    println!("{}", number);
}

fn prints_string(str: String) {
    println!("{}", str);
}

fn main() {
   let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let my_str = "aaaaa".to_string();
    prints_string(my_str.clone());
    prints_string(my_str);

}
