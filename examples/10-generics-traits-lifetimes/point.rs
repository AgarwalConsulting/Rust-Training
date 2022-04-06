#[derive(Debug)]
struct Point<U, V> {
    x: U,
    y: V,
}

fn main() {
    let p = Point{x: 10, y: 12};
    // let p = Point::<i32>{x: 10, y: 12};
    let p = Point::<i32, i32>{x: 10, y: 12};

    println!("Point: {:?}", p);

    let pf = Point{x: 10.12, y: 12.12};
    // let pf = Point::<f64>{x: 10.12, y: 12.12};
    let pf = Point::<f64, f64>{x: 10.12, y: 12.12};

    println!("Point: {:?}", pf);

    let v = Point{x: 12, y: 'A'};
    // let v = Point::<i32>{x: 12, y: 'A'};
    let v = Point::<i32, char>{x: 12, y: 'A'};

    println!("Point: {:?}", v);
}
