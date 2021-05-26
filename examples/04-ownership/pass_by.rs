fn increment(mut x: i32) {
    x += 1 // Only the copy get's incremented
}

fn main() {
    let mut x = 10; // Get's allocated over the stack

    increment(x); // Copied in to increment

    println!("The value of x: {}", x);
}
