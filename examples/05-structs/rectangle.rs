#[derive(Debug)]
struct Rectangle {
    x: i32,
    y: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.x * self.y
    }

    fn scale(&mut self, by: i32) {
        self.x = self.x * by;
        self.y = self.y * by;
    }
}

fn main() {
    let mut r = Rectangle{x: 5, y: 4};

    println!("Rectangle: {:?}", r);

    println!("Area of rectangle: {}", r.area()); // ?

    r.scale(2);

    println!("Scaled rectangle: {:#?}", r);
    println!("Area of scaled rectangle: {}", r.area());
}
