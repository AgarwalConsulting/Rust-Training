use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let mut val = 10;

    let tx = Arc::new(tx);

    {
        let tx1 = tx.clone();
        thread::spawn(move || { // Producer
            for _ in 1..10 {
                thread::sleep(Duration::from_secs(1));
                val += 1;
                tx1.send(val).unwrap();
            }
        });
    }

    {
        let tx2 = tx.clone();
        thread::spawn(|| { // Producer
            for i in 1..10 {
                tx2.send(i).unwrap();
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
