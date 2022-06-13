fn main() {
    let my_vec = vec![2, 3, 4];


    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {}", number);
        }
        // match my_vec.get(index) {
        //     Some(number) => println!("The number is: {}", number),
        //     None => {}
        // }
    }

    let weather_vec = vec![
        vec!["Berlin", "5", "-8"],
        vec!["Athens", "10"],
    ];

    for mut city in weather_vec {
        println!("For the city of {:?}", city.get(0).unwrap());
        while let Some(information) = city.pop() {
           if let Ok(number)  = information.parse::<i32>() {
               println!("The number is: {}", number);
           }
        }
    }
}