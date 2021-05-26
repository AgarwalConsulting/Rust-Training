fn main() {
    let mut t: (i32, char) = (42, "A");

    println!("The value of t: {:#?}", t);

    println!("The first value: {}", t.0);

    t.0 = 10;

    println!("The value of t: {:?}", t);

    let (_, y) = t; // Destructuring

    println!("The value of y: {}", y);
}
