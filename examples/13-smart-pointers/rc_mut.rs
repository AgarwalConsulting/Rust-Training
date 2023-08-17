use std::ops::Drop;
use std::rc::Rc;

#[derive(Debug)]
struct Point (i32, i32);

fn print_double(p : &mut Point) {
    println!("\tX: {} | Y: {}", p.double_x(), p.1 * 2)
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("point is dropped!");
    }
}

impl Point {
    fn double_x(&mut self) -> i32 {
        self.0 *= 2;
        self.0
    }
}

fn main() {
    let mut p = Point(1, 2);

    let mut v1; // L42
    let mut rcv = Rc::new(p); // Move! // L34! // p?

    println!("{}", Rc::strong_count(&rcv));

    // v1 = Rc::clone(&rcv); // Doesn't clone the underlying vector // Type: Rc<Point> // L41

    v1 = Rc::get_mut(&mut rcv);

    // let v2 = Rc::clone(&rcv); // L34

    if let Some(v1) = v1 {
        print_double(v1); // Move!
    }

    println!("Main exists here!");

    println!("{:?}", rcv);
    println!("{}", Rc::strong_count(&rcv));
}
