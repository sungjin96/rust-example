fn main() {
    string_and_str();
}

// 14 강
fn string_and_str() {
    // String = Sized type -> heap
    // &str = Dynamic type
    let _my_name = "SJ"; // &str
    let _other_name = String::from("SJ");

    let mut my_other_name = "SJ".to_string();
    my_other_name.push('!');
    println!("{my_other_name}");
}
