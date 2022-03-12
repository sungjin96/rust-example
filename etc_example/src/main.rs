fn match_colors(rgb: (u32, u32, u32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Every colors has at least 10")
    }
}


fn main() {
    match_colors((1, 2, 3))
}
