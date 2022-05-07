enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

enum Season {
    Spring,
    Summer,
    Autumn,
    Winter
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*;

    match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    }
}

fn main() {
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);

    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("The number is {}", season as u32);
    }
}