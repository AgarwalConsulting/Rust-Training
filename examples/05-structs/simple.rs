#[derive(Debug)]
struct Person {
    name: String,
    age: i32
}

impl Person {
    fn new(name: String) -> Self {
        Person{name, age: 30}
    }
}

fn main() {
    let p = Person::new(String::from("Gaurav"));

    println!("value of p: {:?}", p);

    // // let p = Person{name: String::from("Gaurav"), age: 30};
    // let (name, age) = (String::from("Gaurav"), 30);

    // // let p = Person{name: name, age: age};
    // let p = Person{name, age};

    // println!("Name: {}", p.name);
    // println!("Age: {}", p.age);

    // println!("Person: {:#?}", p);

    // // let mut p1 = p; // Move

    // // p1.name = String::from("Ranjith");

    // let p1 = Person{
    //     // name: String::from("Ranjith"),
    //     age: 29,
    //     ..p // Struct update syntax
    // };

    // // println!("Person: {:#?}", p);
    // println!("Person 1: {:#?}", p1);
}
