// tuples
// Destructuring
// Structure

fn main() {
    let my_tuple = (8, "SJ", vec![9, 4]);
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [7, 8, 9], 7.7);

    println!("{:?}", my_tuple);
    println!(
        "First item: {:?}
Second item: {:?}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    );

    let my_vec = vec![("asd", 10), ("Hey", 9)];
    println!("{:?}", my_vec);

    let str_tuple = ("one", "two", "three");
    let str_array = ["a", "b", "c"];

    let (a, b, c) = str_tuple;
    let [a1, b1, c1] = str_array;

    println!("a: {a}, b: {b}, c: {c}");
    println!("a1: {a1}, b1: {b1}, c1: {c1}");
}