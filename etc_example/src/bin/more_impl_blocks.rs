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
    let mut my_animal = Animal::new_dog(10);

    my_animal.print();
    my_animal.add_age();
    my_animal.print();
    my_animal.change_to_animal(AnimalType::Cat);
    my_animal.print();

}