fn main() {
    let x: i32;

    x = 42;

    println!("The value of x is {}", x);

    let x = 12.10; // Shadowing!

    println!("The value of x is {}", x);
}
