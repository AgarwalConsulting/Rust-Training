fn get_greeting() -> &String {
    let s = String::from("hello");

    return &s
}

fn main() {
    let s = get_greeting();

    println!("The value of s: {}", s);
}
