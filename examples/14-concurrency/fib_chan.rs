use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender};

fn fib(n: usize, sender: Sender<u64>) -> u64 {
    let mut counter = 0 as usize;
    let (mut i, mut j) = (0, 1);

    loop {
        if counter == n {

            return i
        }

        thread::sleep(Duration::from_millis(100));

        if let Err(_) = sender.send(i) {
            println!("Terminating fibonacci!");
            return 0
        }
        // av.lock().as_mut().unwrap().push(i);

        (i, j) = (j, i+j);
        counter+= 1;
    }
}

fn main() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || fib(100, sender));

    thread::sleep(Duration::from_millis(1000));

    loop {
        if let Ok(val) = receiver.recv() {
            println!("\tReceived: {}", val);
        } else {
            break
        }
    }

    println!("Exiting main!");
}
