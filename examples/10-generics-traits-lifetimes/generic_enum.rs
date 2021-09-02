#[derive(Debug)]
enum Action<T, U> {
    Jump(T),
    Stop,
    Turn(U),
    Walk,
}

fn main() {
    let mut a: Action<f64, bool> = Action::Jump(10.12);
    println!("Value: {:?}", a);

    a = Action::Turn(false);

    println!("Value: {:?}", a);

    a = Action::Turn(12.12);
    println!("Value: {:?}", a);

    let a: Action<i32, bool> = Action::Jump(10);
    println!("Value: {:?}", a);

    let a: Action<String, ()> = Action::Jump(String::from("High"));
    println!("Value: {:?}", a);
}
