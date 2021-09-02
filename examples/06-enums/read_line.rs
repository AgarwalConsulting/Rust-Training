use std::io;

fn main() {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(no_of_bytes_read) => println!("Received: {:?}", no_of_bytes_read),
        Error => println!("Encountered error"),
    };

    let input: Result<i32, _> = input.trim().parse();

    match input {
        Ok(number) => println!("Input was: {}", number),
        Error => println!("Unable to parse!")
    }

    // println!("Input was: {:?}", input)
}
