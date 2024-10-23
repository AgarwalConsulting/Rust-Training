#[derive(Debug)]
pub struct Cube(pub i32);

// impl super::Shape3D for Cube;

impl super::Shape3D for Cube {
    fn volume(&self) -> i32 {
        self.0 * self.0 * self.0
    }

    fn area(&self) -> i32 {
        self.0 * self.0
    }
}

impl super::Shape for Cube {
}
