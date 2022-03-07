// tuples

fn main() {
    let my_tuple = (8, "SJ", vec![8, 9, 10]);

    println!("{:?}", my_tuple);
    println!("{:?}", my_tuple.0);
    println!("{:?}", my_tuple.1);
    println!("{:?}", my_tuple.2);

    let (a,b,c) = my_tuple;
}
