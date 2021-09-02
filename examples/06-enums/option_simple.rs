// enum Option<T> {
//     Some<T>,
//     None
// }

fn increment(i: Option<i32>) -> i32 {
    // match i {
    //     Some(i) => i+1,
    //     _ => 0,
    // }
    if let Some(x) = i {
        return x + 1
    };

    return 0
}

fn main() {
    let some_number = Some(5);
    let no_number: Option<i32> = None;
    // let some_number: Option<i32> = Some(5);

    println!("{:?}", increment(some_number));
    // println!("{:?}", no_number);
    println!("{:?}", increment(no_number));
}
