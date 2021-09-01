// #[derive(Debug, Copy, Clone)]
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

fn main() {
    let p1 = Point{x: 10.12, y: 42.12};

    // let mut p2 = p1;

    // println!("{:?}", p2);

    // p2.x = 12.0;

    let p2 = Point{
        x: 12.0,
        ..p1
    };
    println!("{:?}", p2);

    println!("{:?}", p1);
}
