use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // let l = List::Nil;
    let a = Rc::new(Cons(4, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("count creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(2, Rc::clone(&a));
        println!("count creating c = {}", Rc::strong_count(&a));

        println!("{:?}", c);
    }

    println!("count after c = {}", Rc::strong_count(&a));

    println!("{:?}", b);
}
