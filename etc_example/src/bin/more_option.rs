fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

// .unwrap() => 만약 Option이 None일때 실행을 안하고 다음으로 넘어간다.
// .expect() => 파라미터로 에러 메세지를 넣을 수 있음.

fn main() {
    let new_vew = vec![1, 2, 3, 4];
    let index = take_fifth(new_vew);

    index.expect("Needed at least five items - make sure Vec has at least five");

    match index {
        Some(number) => println!("I got a number: {}", number),
        None => println!("There was nothing inside")
    };

    if index.is_some() {
        println!("My Number is a {}", index.unwrap());
    };
}