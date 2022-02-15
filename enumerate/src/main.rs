enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 메소드 내용은 여기 정의할 수 있습니다.
    }
}


enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;
}

