fn add_by(by: i32) -> Box<dyn Fn(i32) -> i32> {
    return Box::new(move |x: i32| -> i32 {
        x + by
    })
}

fn main() {
    let add_by_2 = add_by(2);

    println!("{}", add_by_2(10));
    println!("{}", add_by_2(12));
    println!("{}", add_by_2(100));
}
