#[derive(Debug)]
struct Person {
    name: String,
    age: i32
}

// fn add_last_name(s: &mut String, last_name: String) {
//     s.push_str(&format!(" {}", last_name))
// }

impl Person {
    fn new(name: String, age: i32) -> Self {
        Person{name, age}
    }
}

// p.add_last_name(String::from("Agarwal"));

impl Person {
    fn add_last_name(&mut self, last_name: String) {
        self.name.push_str(&format!(" {}", last_name))
    }
}

fn main() {
    let name = String::from("Gaurav");
    let age = 30;
    // let mut p = Person{name: name, age: age};
    // let mut p = Person{name, age};
    let mut p = Person::new(name, age);
    // let mut p = Person{String::from("Gaurav"), 30};

    println!("{:?}", p);

    println!("Name of the person: {}", p.name);

    // p.age = 31;

    println!("{:?}", p);

    // p.name.push_str(" Agarwal");
    // add_last_name(&mut p.name, String::from("Agarwal"));

    p.add_last_name(String::from("Agarwal"));

    // let p1 = p;

    // println!("{:?}", p1);

    println!("{:?}", p);
}
