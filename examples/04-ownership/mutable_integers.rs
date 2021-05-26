fn increment(x: &mut i32) {
    *x += 1 // Only the copy get's incremented
}

fn main() {
    let mut x = 10; // Get's allocated over the stack

    increment(&mut x); // Borrow!

    println!("The value of x: {}", x);
}
