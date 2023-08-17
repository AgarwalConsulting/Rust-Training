use std::ops::Drop;
// use std::rc::Rc;

#[derive(Debug)]
struct Point (i32, i32);

// fn print_double(p : Rc<Point>) {
//     println!("\tX: {} | Y: {}", p.0 * 2, p.1 * 2)
// }

// impl Drop for Point {
//     fn drop(&mut self) {
//         println!("point is dropped!");
//     }
// }

fn main() {
    let mut p = Point(1, 2);

    let v1;
    {
        let p2 = p; // Move! L: 25!

        v1 = &p2;
    }

    println!("v1: {:?}", v1);
}
