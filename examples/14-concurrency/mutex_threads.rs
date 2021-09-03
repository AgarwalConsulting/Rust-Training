use std::{sync::{Mutex, Arc}, thread};
// use std::rc::Rc;

fn main() {
    let m = Arc::new(Mutex::new(10));

    let m1 = Arc::clone(&m);
    let m2 = Arc::clone(&m);

    let handle = thread::spawn(move || {
        let mut num = m1.lock().unwrap();
        *num = *num * 2;
    });

    handle.join().unwrap();

    println!("{:?}", m2);
}
