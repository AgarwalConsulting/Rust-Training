use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut v = vec![1, 2, 3];
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1));

        tx.send(4).unwrap();
    });

    // drop(v); // oh no!

    // handle.join().unwrap();

    let received = rx.recv().unwrap();

    v.push(received);

    println!("{:?}", v);
}
