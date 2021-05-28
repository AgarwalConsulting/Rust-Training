fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    // let f: fn(i32, i32) -> i32 = add;
    let mut f: Box<dyn Fn(i32, i32) -> i32>;

    f = Box::new(|x, y| -> i32 {
        x + y
    });

    // let f = add;

    // println!("{}", f);

    println!("{}", f(1, 2));

    f = Box::new(|x, y| -> i32 {
        x * y
    });

    println!("{}", f(10, 2));
}
