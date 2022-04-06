use scratchpad::point::*;
use scratchpad::pair::*;
use std::fmt::Debug;

use scratchpad::my_traits::Firstable;

fn print_first<T>(v: T)
    where T: Firstable {
    println!("\tFirst value: {}", v.first())
}

#[derive(Debug)]
struct Dummy;

fn main() {
    let p: Point<i32> = Point{x: 42, y: 10};

    println!("Point: {:?}", p);
    print_first(p);

    let p: Point<Dummy> = Point{x: Dummy{}, y: Dummy{}};
    // let p = Point{x: Dummy{}, y: Dummy{}};

    println!("Dummy Point: {:?}", p);
    print_first(p);

    let pa = Pair("Gaurav", "Chennai");

    println!("Pair: {:?}", pa);
}
