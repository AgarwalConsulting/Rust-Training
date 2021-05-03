// From: https://doc.rust-lang.org/book/ch03-05-control-flow.html
// Listing 3-3: Using a while loop to run code while a condition holds true

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
