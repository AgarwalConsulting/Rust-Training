#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Person {
        Person{name, age}
    }
}

pub fn greeting(p: Person) {
    println!("Hello, {}!", p.name);
}
