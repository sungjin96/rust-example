fn main() {
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "sold") => println!("It's not very nice today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", _) => println!("Cloudy and something else"),
        _ => println!("Not sure what the weather is.")
    }

    let children = 5;
    let married = true;

    match (children, married) {
        (children, married) if !married => println!("Not married with {} children ", children),
        (c, m) if m && c == 0 => println!("Married nut with no children"),
        _ => println!("Some other type of marriage an children combination")
    }
}