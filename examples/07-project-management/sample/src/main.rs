// mod people;
// mod greet;

// extern rand;
use rand::prelude::*;
// use sample::people as p;
// use greet;

use sample::{people as p, greet::people as g};
// use sample::greet;

fn main() {
    let p1 = p::Person{name: String::from("Gaurav"), age: 30};

    println!("{:?}", p1);

    // p::call(p1);

    // println!("{}", greet::greeting(String::from("Gaurav")));

    println!("{}", g::greeting(p1));

    println!("{}", random::<i32>());
}
