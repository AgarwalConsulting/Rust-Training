fn main() {
    let b: bool = true;

    let i: i32 = 10;
    let j: f64 = 9;

    b = i == j;

    println!("The value is: {}", b);

    let i = 1;

    if i != 0 {
        println!("i is truthy!");
    }
}
