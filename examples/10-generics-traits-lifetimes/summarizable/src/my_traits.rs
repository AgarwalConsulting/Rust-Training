use super::tweet::*;

pub trait Summarizable {
    fn summarize(&self) -> String {
        String::from("...")
    }
}

// impl Summarizable for Tweet {}

impl Summarizable for Tweet {
    fn summarize(&self) -> String {
        self.summarize()
    }
}
