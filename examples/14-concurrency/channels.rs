use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        println!("Sending...");
        let val = 42;
        thread::sleep(Duration::from_secs(3));
        tx.send(val).unwrap();
        println!("Sent: {}", val);
    });

    println!("Receiving...");
    thread::sleep(Duration::from_secs(3));
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
