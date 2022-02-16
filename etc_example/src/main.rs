// mutability
// shaowing 같은 이름을 다시 쓰는 것

fn double(input: i32) -> i32 {
    input * 2
}

fn triple(input: i32) -> i32 {
    input * 3
}

fn main() {
    let x = 9;
    let x = double(x);
    let x = triple(x);

    println!("x is value : {}", x);

    let my_variable = 9;
    {
        let my_variable = "Some string";
        println!("{}", my_variable);
    }

    println!("{}", my_variable)
}
