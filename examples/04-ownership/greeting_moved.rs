fn greeting(mut s: String) {
    let greet = String::from("Hello, ");
    s.insert_str(0, &greet);
}

fn main() {
    let s = String::from("World");

    greeting(s);

    println!("The value of s: {}", s);
}
