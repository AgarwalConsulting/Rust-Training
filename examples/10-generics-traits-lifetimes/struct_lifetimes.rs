#[derive(Debug)]
struct Person<'a> {
    name: &'a String,
    age: u8
}

fn main() {
    let mut p = Person{name: &String::from("Gaurav"), age: 30};

    {
        let name = String::from("G A");
        p.name = &name
    }

    println!("{:?}", p)
}
