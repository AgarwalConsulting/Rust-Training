use super::tweet::*;

pub trait Summarizable {
    // fn dup(&self, content: String) -> Self;
    // fn summarize<T>(&self) -> T;

    fn summarize(&self) -> String {
        String::from("...")
    }
}

impl Summarizable for Tweet {
    // fn dup(&self, content: String) -> Tweet {
    //     Tweet{content, ..*self}
    // }

    fn summarize(&self) -> String {
        self.summarize()
    }
}
