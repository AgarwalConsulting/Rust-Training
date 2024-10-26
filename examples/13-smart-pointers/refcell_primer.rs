use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(5);
    // let old_value = cell.replace(6);

    cell.replace(6);

    // assert_eq!(old_value, 5);
    // assert_eq!(cell, RefCell::new(6));

    println!("Look ma! cell wasn't declared mutable: {:?}", cell);
}
