use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    // let (c2_tx, c2_rx) = mpsc::channel();

    // Fibonacci Producer
    thread::spawn(|| {
        let (mut i, mut j) = (-1, 1);

        loop {
            let temp = i;
            i = j;
            j = j + i;
            // (i, j) = (j, j +i);

            println!("Sending... {}", j);
            tx.send(j).unwrap();
            println!("Sent");
        }
    });

    // Consumer
    for _ in (1..10) {
        thread::sleep(Duration::from_millis(1000));
        println!("Receiving...");
        let received = rx.recv().unwrap();

        println!("Received: {}", received);
    }
}
