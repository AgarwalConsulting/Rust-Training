use std::thread;
use std::time::Duration;

fn main() {
    let mut v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1));

        v.push(4);

        println!("Here's a vector: {:?}", v);
    });

    // drop(v); // oh no!

    handle.join().unwrap();

    println!("{:?}", v);
}
