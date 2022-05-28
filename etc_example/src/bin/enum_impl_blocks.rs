#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat(String),
    Dog(String),
}

impl AnimalType {
    fn check_type(&self) {
        match self {
            AnimalType::Cat(name) => println!("Animal type is cat"),
            AnimalType::Dog(name) => println!("Animal type is dog"),
        }
    }

    fn get_name(&self) {
        match self {
            AnimalType::Cat(name) => println!("Animal Type is cat and name is: {}", name),
            AnimalType::Dog(name) => println!("Animal type is dog and name is: {}", name),
        }
    }
}

impl Animal {
    fn new(age: u8, animal_type: AnimalType) -> Self {
        Self {
            age,
            animal_type,
        }
    }

    fn new_cat(age: u8) -> Self {
        Self::new(age, AnimalType::Cat("Windy".to_string()))
    }

    fn new_dog(age: u8) -> Self {
        Self::new(age, AnimalType::Dog("Bab".to_string()))
    }

    fn print(&self) {
        println!("I am a : {:?}", self);
    }

    fn add_age(&mut self) {
        self.age += 1;
    }

    fn change_to_animal(&mut self, animal_type: AnimalType) {
        self.animal_type = animal_type;
    }
}

fn main() {
    use AnimalType::*;
    let mut my_animal = Animal::new_dog(10);
    my_animal.animal_type.get_name();
}