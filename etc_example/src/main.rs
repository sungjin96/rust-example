fn loop_then_return(mut counter: i32) -> i32{
   loop {
       counter += 1;
       if counter % 50 == 0 {
           break;
       }
   }
    counter
}

fn main() {
    let my_number;
    {
        let x = loop_then_return(0);
        my_number = x
    };
    println!("{}", my_number);
}
