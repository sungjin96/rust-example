fn main() {
    print!(r#"c:\thisdrice\new_drive"#);
    println!("
let me tell you
어떤 이야기를
봅시다.
             ");

    let my_variable = 9;
    // p = pointer, b = 2byte 등등..
    println!("{:p}", &my_variable);


    let title = "TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}
