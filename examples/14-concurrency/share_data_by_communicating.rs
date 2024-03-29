use std::thread;
use std::sync::mpsc;
// use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let mut val = 10;

    // let tx = Arc::new(Mutex::new(tx));

    {
        // let mtx1 = Arc::clone(&tx);
        let tx = tx.clone();
        thread::spawn(move || { // Producer
            for _ in 1..10 {
                thread::sleep(Duration::from_millis(1));
                val += 1;
                // let tx = mtx1.lock().unwrap();
                tx.send(val).unwrap();
            }
        });
    }

    {
        // let mtx2 = Arc::clone(&tx);
        let tx = tx.clone();
        thread::spawn(move || { // Producer
            for i in 1..10 {
                thread::sleep(Duration::from_millis(1));
                // let tx = mtx2.lock().unwrap();
                tx.send(i).unwrap();
            }
        });
    }

    let h2 = thread::spawn(move || { // Consumer
        for _ in 1..10 {
            // thread::sleep(Duration::from_secs(2));
            println!("Value: {}", rx.recv().unwrap());
        }
    });

    h2.join().unwrap();
}
