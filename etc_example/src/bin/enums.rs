#[derive(Debug)]
enum ThingsInTheSky {
    Sun,
    Stars,
}

fn create_sky_state(time: u8) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars
    }
}

fn check_sky_state(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun"),
        ThingsInTheSky::Stars => println!("I can see the stars"),
    }
}

fn main() {
    let sky = create_sky_state(4);

    check_sky_state(&sky);
    println!("{:#?}", sky);
}
