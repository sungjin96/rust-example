#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl Animal {
    fn new(age: u8, animal_type: AnimalType) -> Self {
        Self {
            age,
            animal_type
        }
    }

    fn new_cat(age: u8) -> Self {
        Self::new(age, AnimalType::Cat)
    }

    fn new_dog(age: u8) -> Self {
        Self::new(age, AnimalType::Dog)
    }
}

fn main() {
    let my_vec = vec![Animal::new_cat(8), Animal::new_cat(30)];

    println!("I made a: {:?}", my_vec[0]);
}