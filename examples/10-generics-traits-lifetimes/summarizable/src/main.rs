use scratchpad::news::*;
use scratchpad::tweet::*;

// use scratchpad::my_traits;
use scratchpad::my_traits::Summarizable;

// fn print_summary<T: my_traits::Summarizable>(a: T) {
fn print_summary<T: Summarizable>(a: T) {
// fn print_summary(a: impl Summarizable) {
// fn print_summary<T: Summarizable + Copy>(a: T) {
    println!("\tSummary: {}", a.summarize());
}

// impl Summarizable for Tweet {
//     fn summarize(&self) -> String {
//         self.summarize()
//     }
// }

fn main() {
    let na = NewsArticle{title: String::from("Rust version 1.59 is out"), content: String::from("version 1.59 brings new improvements."), author: String::from("Steve")};

    println!("News Article: {:?}", na);
    print_summary(na);

    let tweet = Tweet{content: String::from("Go is awesome!"), handle: String::from("algogrit")};

    println!("Tweet: {:?}", tweet);

    print_summary(tweet);
}
