use std::fmt::Debug;

trait Doubler: Sized {
    type T;

    fn double(&self) -> Self::T;
}

#[derive(Debug)]
struct Point(i32, i32);

impl Doubler for Point {
    type T = Self;

    fn double(&self) -> Self::T {
        Point(self.0 * 2, self.1 * 2)
    }
}

fn main() {
    let p = Point(10, 12);

    println!("{:?}", p);

    let p2 = p.double();

    println!("{:?}", p2);
}
