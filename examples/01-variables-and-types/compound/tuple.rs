fn main() {
    let mut t: (i32, char) = (42, 'A');

    println!("The value of t: {:#?}", t);

    let v = t.0;

    println!("The first value: {}", v);

    t.0 = 10;

    println!("The value of t: {:?}", t);

    let (_, y) = t; // Destructuring

    println!("The value of y: {}", y);
}
