use crate::{people, greet as g};

pub fn create_person_and_greet(name: String, age: u8) {
    // let p = super::super::people::Person::new(name, age);
    let p = people::Person::new(name, age);

    // println!("{}", super::greeting(p.name));
    println!("{}", g::greeting(p.name));
    // people::greeting(p);
}
