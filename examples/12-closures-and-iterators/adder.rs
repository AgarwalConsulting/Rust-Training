// fn adder(by: i32) -> Fn(i32) -> i32 {
//     return |x: i32| -> i32 {
//         x + by
//     }
// }

// fn adder<T>(by: i32) -> T
//     where T: Fn(i32) -> i32 {
//     return |x: i32| -> i32 {
//         x + by
//     }
// }

fn adder(by: i32) -> Box<dyn Fn(i32) -> i32> {
    return Box::new(move |x: i32| -> i32 {
        x + by
    })
}

fn main() {
    let add_by_2 = adder(2);

    let add_by_10 = adder(10);

    println!("{}", add_by_2(10)); // 12
    println!("{}", add_by_2(20)); // 22
    println!("{}", add_by_2(30)); // 32

    println!("{}", add_by_10(30)); // 40
}
