use std::fmt::Display;

fn longest<'a, T>(s1: &'a str, s2: &'a str, ann: T) -> &'a str
    where T: Display {
    println!("\tAnnouncing: {}", ann);
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let s1 = String::from("hello");

    let result;

    // {
    let s2 = String::from("Namaste");

    result = longest(&s1, &s2, String::from("Hey, Mukund!"));
    // }

    println!("The largest is: {}", result);
}
