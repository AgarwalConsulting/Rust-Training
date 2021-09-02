// use simple::greet;
// use simple::people;
use rand;

// use simple::{greet, people};
use simple::*;
// use simple::people;
// use crate::greet;

fn main() {
    // println!("Hello, world!");
    println!("{}", greet::greeting(String::from("World")));

    // greet::people::create_person_and_greet(String::from("Gaurav"), 30);
    create_person_and_greet(String::from("Gaurav"), 30);

    println!("Random number: {}", rand::random::<i32>());

    // println!("{:?}", people::Person{name: String::from("Gaurav"), age: 30});
}
