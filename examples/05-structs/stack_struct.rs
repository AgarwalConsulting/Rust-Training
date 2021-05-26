#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let p = Point{x: 10, y: 12};

    println!("point: {:?}", p);

    let p1 = p; // Moved

    println!("point: {:?}", p);

    println!("point 1: {:?}", p1);
}
