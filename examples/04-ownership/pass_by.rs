fn increment(mut x: i32) {
    x += 1
}

fn main() {
    let mut x = 10;

    increment(x);

    println!("The value of x: {}", x);
}
