fn main() {
    let s = String::from("World");

    println!("The value of s: {}", s);

    let s2 = s; // Ownership is transferred => moved

    println!("The value of s2: {}", s2);

    println!("The value of s: {}", s);
}
