use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::sync::Arc;

fn main() {
    let m = Arc::new(Mutex::new(10));

    let m1 = m.clone();
    let m2 = m.clone();

    thread::spawn(move || {
        let mut num = m1.lock().unwrap();
        *num += 1;
    });

    thread::sleep(Duration::from_secs(1));

    let num = m2.lock().unwrap();
    println!("Value: {}", *num);
}
