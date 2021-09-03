// fn increment(x: i32) -> i32 {
//     x + 1
// }

fn main() {
    let increment = |x: i32| -> i32 {
        x + 1
    };

    println!("{}", increment(10));
}
