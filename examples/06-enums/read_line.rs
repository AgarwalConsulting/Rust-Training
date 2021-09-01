use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input: Result<i32, _> = input.trim().parse();

    println!("Input was: {:?}", input)
}
