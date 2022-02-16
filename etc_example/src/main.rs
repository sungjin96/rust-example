fn main() {
    let my_number = 10;
    let single_refernce = &my_number;
    let double_reference = &single_refernce;
    let five_reference = &&&&&my_number;
}
