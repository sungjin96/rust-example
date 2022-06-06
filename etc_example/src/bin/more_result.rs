use std::num::ParseIntError;

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("".to_string())
    }
}

fn parse_number(number: &str) -> Result<i32, ParseIntError> {
    number.parse()
}

fn main() {
    let mut result_vec = Vec::new();
    let mut result_parse_vec = vec![];

    for number in 1..=7 {
        result_vec.push(check_if_five(number));
    }

    println!("{:#?}", result_vec);

    result_parse_vec.push("5");
    result_parse_vec.push("aa5");

    for str in result_parse_vec {
        println!("{:#?}", parse_number(str));
    }

}