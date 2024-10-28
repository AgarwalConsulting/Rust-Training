use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn fib(n: usize, av: Arc<Mutex<Vec<u64>>>) -> u64 {
    let mut counter = 0 as usize;
    let (mut i, mut j) = (0, 1);

    loop {
        if counter == n {

            // println!("Next Val: {}", i);
            return i
        }

        thread::sleep(Duration::from_millis(100));

        av.lock().as_mut().unwrap().push(i);

        (i, j) = (j, i+j);
        counter+= 1;
    }
}

fn main() {
    let v: Vec<u64> = Vec::new();

    let av = Arc::new(Mutex::new(v));

    let tav = Arc::clone(&av);

    thread::spawn(move || fib(10, tav));

    let mav = Arc::clone(&av);
    loop {
        thread::sleep(Duration::from_millis(1000));
        println!("{:?}", mav.lock().unwrap());
    }
}
