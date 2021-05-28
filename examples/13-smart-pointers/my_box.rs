use std::fmt::Debug;
use std::ops::Deref;
use std::ops::Drop;

struct MyBox<T: Debug>(T);

impl<T: Debug> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Debug> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Debug> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping value: {:?}", self.0)
    }
}

fn main() {
    let b = 5;
    let b = MyBox::new(5);

    println!("{}", *b + 1);
    println!("{}", b.deref() + 1);
    drop(b);
    println!("Exiting main!");

    // b.drop();
}
