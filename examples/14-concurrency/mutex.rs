use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5); // Interior-mutability

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
