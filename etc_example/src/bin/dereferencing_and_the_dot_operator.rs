struct Item {
    number: u8,
}

impl Item {
    fn compare_number(&self, other_number: u8) {
        println!("Are they equal? {}", self.number == other_number);
    }
}

fn main() {
    let my_number = 10;
    let reference = &my_number;
    println!("Are they the same? {}", my_number == *reference);

    let item = Item {
        number: 10
    };
    let item_ref = &item;

    item.compare_number(10);
    item_ref.compare_number(10);
}