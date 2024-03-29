fn main() {
    // let f: f64 = 42.12;

    // let i: i32;

    // i = f as i32; // Type conversion; drops precision!

    // println!("The value of i: {}", i)

    // i = f.ceil() as i32;
    // i = f.floor() as i32;
    // i = f.round() as i32;

    let x: i32 = 1244363;
    let y: i8;

    // y = x as i8; Truncates bits!
    y = x.into(); // Causes compile time check

    println!("The value of y: {}", y);
}
