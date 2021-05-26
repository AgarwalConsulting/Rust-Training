#[derive(Debug)]
struct Square(i32);

impl Square {
    fn area(&self) -> i32 {
        self.0 * self.0
    }
}

impl Square {
    fn scale(&mut self, by: i32) {
        self.0 = self.0 * by
    }
}

fn main() {
    let mut sq = Square(10);

    println!("Square is: {:?}", sq);

    println!("Area of square: {}", sq.area()); // Square gets borrowed

    sq.scale(2);

    println!("Area of square: {}", sq.area()); // Square gets borrowed
}
