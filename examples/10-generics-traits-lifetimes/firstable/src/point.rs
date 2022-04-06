use super::my_traits::Firstable;
// use std::fmt::Display;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Firstable for Point<T>
    where T: Debug {
    fn first(&self) -> String {
        format!("{:?}", self.x)
    }
}
