use std::collections::HashMap;
use std::collections::BTreeMap;
// Other collection types
// HashMap, BTreeMap
// key, value

struct City {
    name: String,
    population: HashMap<u32, u32>,
    // year + population
    population_b: BTreeMap<u32, u32>,
}

fn main() {
    let mut tallin = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
        population_b: BTreeMap::new(),
    };

    tallin.population.insert(1372, 3_250);
    tallin.population.insert(1851, 24_000);
    tallin.population.insert(2020, 437_619);

    for (year, population) in tallin.population {
        println!("In the year {} the population was {}", year, population);
    }
}