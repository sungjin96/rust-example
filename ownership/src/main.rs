fn main() {
    // this is area of stack
    let mut stack_string = "hello";
    println!("stack: {}", stack_string);

    // this is area of heap
    let mut heap_string  = String::from("hello");
    heap_string.push_str(", world!");
    println!("heap: {}", heap_string);

    let x = 5;
    let _y = x;

    let s1 = String::from("hello");
    let _s2 = s1;
}
