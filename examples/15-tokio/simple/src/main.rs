use std::thread;
use std::time::Duration;

// fn main() {
//     // println!("Hello, world!");

//     let handle = thread::spawn(|| {
//         let  (mut i, mut j) = (0 as u64, 1 as u64);
//         loop {
//             println!("number: {}", i);
//             (i, j) = (j, i+j);
//             thread::sleep(Duration::from_millis(100));
//         }
//     });

//     // handle.join();

//     thread::sleep(Duration::from_millis(3_000));
// }

async fn get_nth_fib(n: i32) -> u64 {
    let mut counter = 0;
    let  (mut i, mut j) = (0 as u64, 1 as u64);

    loop {
        counter += 1;

        if n == counter {
            return i;
        }

        (i, j) = (j, i+j);
    }
}

#[tokio::main]
async fn main() {
    let n = 10;

    let handle = tokio::spawn(get_nth_fib(n));

    // tokio::join!() - For joining multiple futures

    let nth_fib = handle.await.unwrap();

    println!("The {n}th fib number is: {nth_fib}");
}
