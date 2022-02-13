fn main() {
    let s = String::from("hello world");
    let word = first_word(&s[..]);
    println!("word is {}", word);

    let a = String::from("hello world");
    let hello = &a[0..5]; // &a[..5]이랑 같은 표현
    let world = &a[6..11]; // &a[6..a.len()]이랑 같은 표현

    println!("hello is {}", hello);
    println!("world is {}", world);

    let b = [1,2,3,4,5];
    let _slice = &b[1..3];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
