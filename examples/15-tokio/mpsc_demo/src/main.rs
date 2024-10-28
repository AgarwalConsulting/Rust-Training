use tokio;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::{self, Sender};

async fn fib(n: usize, sender: Sender<u64>) -> u64 {
    let mut counter = 0 as usize;
    let (mut i, mut j) = (0, 1);

    loop {
        if counter == n {

            return i
        }

        println!("Sending...");
        if let Err(_) = sender.send(i).await {
            println!("Terminating fibonacci!");
            return 0
        }
        // av.lock().as_mut().unwrap().push(i);

        (i, j) = (j, i+j);
        counter+= 1;
    }
}

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::channel(100); // Buffer size: <>

    tokio::spawn(fib(10, sender));

    thread::sleep(Duration::from_millis(1000));

    loop {
        thread::sleep(Duration::from_millis(100));

        if let Some(val) = receiver.recv().await {
            println!("\tReceived: {}", val);
        } else {
            break
        }
    }

    println!("Exiting main!");
}
