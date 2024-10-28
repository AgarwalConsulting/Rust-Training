use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(5);
    // let cell = RefCell::new(String::from("Hello, "));

    let old_value = cell.replace(6);

    // cell.replace_with(|mut x| {
    //     x.push_str("World!");
    //     x.to_string()
    // });

    // let mut val = cell.borrow_mut();
    // val.push_str(", World!");


    // cell.replace((*cell).push_str(" World!"));
    // cell.replace(6);

    // assert_eq!(old_value, 5);
    // assert_eq!(cell, RefCell::new(6));

    println!("Look ma! cell wasn't declared mutable: {:?}", cell);
}
