use std::ops::Deref;

fn main() {
    let b = 5;
    let b = Box::new(5);

    // println!("{}", *b + 1);
    println!("{}", b.deref() + 1);
}
