fn main() {
    string_method();
}

// 15 강
// capacity / push / push_str / pop / with_capacity
fn string_method() {
    let mut my_name = String::with_capacity(4);
    println!("{}", my_name.capacity());
    my_name.push_str("SJ");
    println!("{}", my_name.capacity());
    my_name.push('!');
    println!("My name is {}", my_name);
}

// 14 강
fn _string_and_str() {
    // String = Sized type -> heap
    // &str = Dynamic type
    let _my_name = "SJ"; // &str
    let _other_name = String::from("SJ");

    let mut my_other_name = "SJ".to_string();
    my_other_name.push('!');
    println!("{my_other_name}");
}
