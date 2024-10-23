#[derive(Debug)]
pub struct Square (pub i32);

impl super::Shape for Square { // external trait for local type
    fn area(&self) -> i32 {
        self.0 * self.0
    }
}
