fn main() {
    // let x = &mut [0, 1, 2];
    let mut x = vec!(0, 1, 2);

    println!("{:?}", x);

    if let Some(elem) = x.get_mut(1) {
        *elem = 42;
    }

    println!("{:?}", x);
}
