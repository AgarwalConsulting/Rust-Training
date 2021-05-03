fn main() {
    let mut counter : i32 = 0;
    loop {
        if counter < 10 {
            counter += 1;
            continue
        }
        break
    }

    println!("The value of counter is {}", counter)
}
