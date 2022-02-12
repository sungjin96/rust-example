fn main() {
    // this is area of stack
    let mut stack_string = "hello";
    println!("stack: {}", stack_string);

    // this is area of heap
    let mut heap_string = String::from("hello");
    heap_string.push_str(", world!");
    println!("heap: {}", heap_string);

    let x = 5;
    let _y = x;

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    let mut s = String::from("hello");

    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world") is Error
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
