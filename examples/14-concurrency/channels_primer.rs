use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let x = Arc::new(Mutex::new(0));

    let x1 = Arc::clone(&x);

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));

        let mut num = x1.lock().unwrap();

        *num = 42;
    });

    // thread::sleep(Duration::from_secs(10));
    println!("Value of x: {:?}", x);
}
