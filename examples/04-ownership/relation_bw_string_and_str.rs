fn print(s: &str) {
    println!("Value of s is: {}", s);
}

fn main() {
    let s: &String = &String::from("Hello");
    let s1 = "Hello";

    if s == s1 {
        println!("They are equal!");
    }

    print(s);
}
