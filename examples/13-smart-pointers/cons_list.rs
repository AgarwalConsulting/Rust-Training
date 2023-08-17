use std::ops::{Deref, Drop};
use std::fmt::Debug;

#[derive(Debug)]
struct MyBox<T: Debug>(T); // tuple struct

impl<T: Debug> MyBox<T> {
    fn new(val: T) -> Self {
        MyBox(val)
    }
}

impl<T: Debug> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping the value contained in mybox: {:?}", self.0)
    }
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let m = MyBox(42);

    // drop(m);

    // println!("{:?}", m);

    println!("Main exists here!")
}
