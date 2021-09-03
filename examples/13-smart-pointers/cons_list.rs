#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    // let l = List::Nil;

    let list = Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Cons(4, Box::new(List::Nil)))))))));

    println!("{:?}", list);
}
