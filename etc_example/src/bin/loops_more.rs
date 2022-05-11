fn main() {
    let mut counter = 0;
    while counter != 5 {
        counter += 1;
        println!("The counter is now : {counter}");
    }

    // Range ( 0..3 )
    for number in 0..3 {
        println!("The number is now : {number}");
    }

    let mut counter = 5;
    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter;
        }
    };

    println!("my_number is now : {my_number}");
}