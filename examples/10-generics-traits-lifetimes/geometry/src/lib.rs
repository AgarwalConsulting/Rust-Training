pub mod square;
pub mod rectangle;
pub mod cube;

pub trait Shape {
    fn area(&self) -> i32;
}

// pub trait Shape3D: Shape {
pub trait Shape3D {
    fn area(&self) -> i32 {
        panic!("area is unimplemented!");
    }

    fn volume(&self) -> i32;
}

impl Shape for rectangle::Rectangle { // local trait for external type
    fn area(&self) -> i32 {
        self.length * self.breadth
    }
}
