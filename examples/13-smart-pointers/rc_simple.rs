use std::ops::Drop;
use std::rc::Rc;

#[derive(Debug)]
struct Point (i32, i32);

fn print_double(p : Rc<Point>) {
    println!("\tX: {} | Y: {}", p.0 * 2, p.1 * 2)
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("point is dropped!");
    }
}

impl Point {
    fn double_x(&mut self) -> i32 {
        self.0 *= 1;
        self.0
    }
}

fn main() {
    let mut p = Point(1, 2);

    let v1; // L42
    {
        let rcv = Rc::new(p); // Move! // L34! // p?

        println!("{}", Rc::strong_count(&rcv));

        v1 = Rc::clone(&rcv); // Doesn't clone the underlying vector // Type: Rc<Point> // L41

        // let v2 = Rc::clone(&rcv); // L34

        println!("{}", Rc::strong_count(&rcv));

        // print_double(v2); // Move!
    }

    println!("{}", Rc::strong_count(&v1));

    print_double(v1); // Move!

    println!("Main exists here!");
}
