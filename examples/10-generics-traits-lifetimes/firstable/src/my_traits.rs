pub trait Firstable {
    fn first(&self) -> String {
        return String::from("n/a")
    }
}

// use std::fmt::Debug;

// pub trait Firstable {
//     type Item: Debug;
//     fn first(&self) -> &Self::Item;
// }
