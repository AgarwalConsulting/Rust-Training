fn add(x: i32, y: i32) -> i32 {
    let result = if x == 42 {
        42
    } else {
        x + y
    };

    result
}

fn main() {
    let (a, b) = (10.12, 12.10);

    println!("The sum is: {}", add(a as i32, b as i32));

    println!("The sum is: {}", add(42, b as i32));
}
