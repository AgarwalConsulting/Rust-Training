#[derive(Debug)]
struct Person{
    name: String,
}

fn main() {
    let g = Person{name: String::from("Gaurav")};

    println!("{:?}", g);
}
