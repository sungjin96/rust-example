// Option - Maybe there, maybe not
// Result - May not work

// .is_ok()
// .is_err()
fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    if check_error(5).is_ok() {
        println!("It's okay, guys!")
    } else {
        println!("It's an error, guys!")
    }

    match check_error(5) {
        Ok(_) => println!("Okay guys"),
        Err(_) => println!("It's an error")
    }
}