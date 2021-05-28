use std::fmt::Debug;

fn display<T: Debug>(x: T) {
    println!("{:?}", x);
}

// fn largest<T>(i: T, j: T) -> T {
//     if i > j {
//         return i
//     } else {
//         return j
//     }
// }

// fn largest(i: i32, j: i32) -> i32 {
//     if i > j {
//         return i
//     } else {
//         return j
//     }
// }

// fn largest(i: &str, j: &str) -> &str {
//     if i > j {
//         return i
//     } else {
//         return j
//     }
// }

#[derive(Debug)]
struct Point(i32, i32);

fn main() {
    let x = 10.12;

    display(x);

    display("Hello");

    display(Point(10, 12));


    // let (a, b) = (10, 12);

    // println!("The largest is: {}", largest(a, b));

    // let (a, b) = ("hello", "hi");

    // println!("The largest is: {}", largest(a, b));
}
