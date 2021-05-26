fn greeting(s: &mut String) {
    let greet = String::from("Hello, ");
    s.insert_str(0, &greet);
}

fn main() {
    let s = String::from("World");

    greeting(&mut s);

    println!("The value of s: {}", s);
}
