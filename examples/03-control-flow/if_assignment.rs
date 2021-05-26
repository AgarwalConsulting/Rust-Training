fn main() {
    let mut x : i32 = 10;

    x = if x % 2 == 0 { 100 } else { 2 };

    println!("The value of x is {}", x);
}
