use scratchpad::news::*;
use scratchpad::tweet::*;

// use scratchpad::my_traits;
use scratchpad::my_traits::Summarizable;

fn main() {
    let mut v: Vec<Box<dyn Summarizable>> = Vec::new();

    // Trait Object!
    // let mut s: Box<dyn Summarizable>; // Pointer -> Heap!

    let na = NewsArticle{title: String::from("Rust version 1.59 is out"), content: String::from("version 1.59 brings new improvements."), author: String::from("Steve")};

    println!("News Article: {:?}", na);

    v.push(Box::new(na));

    // s = Box::new(na);
    // println!("\tSummary: {}", s.summarize());

    let tweet = Tweet{content: String::from("Go is awesome!"), handle: String::from("algogrit")};

    println!("Tweet: {:?}", tweet);

    v.push(Box::new(tweet));

    // s = Box::new(tweet);
    // println!("\tSummary: {}", s.summarize());

    println!("Printing all summaries...");
    for el in v {
        println!("\tSummary: {}", el.summarize());
    }
}
