#[derive(Debug)]
enum Action {
    Jump(i32),
    Stop,
    Turn(String),
    Walk,
}

fn do_action(a: Action) {
    match a {
        Action::Jump(1) => println!("Jumping a step"),
        Action::Jump(2) => println!("Jumping couple of steps"),
        Action::Jump(3) => println!("Jumping to the third step"),
        // Action::Jump(i) => println!("Jumping {} steps", i),
        Action::Turn(direction) => println!("Facing {}", direction),
        Action::Walk => println!("Walking..."),
        // Action::Stop => (),

        // Action::Stop => println!("Halting!"),
        // _ => println!("{:?}", a),
        _ => (),
    }
}

fn main() {
    let some_action = Action::Jump(10);

    do_action(some_action);

    do_action(Action::Stop);
}
