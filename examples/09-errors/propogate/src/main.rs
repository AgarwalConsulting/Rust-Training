// use std::io::stdin;
use std::io::stdin;
use std::num::ParseIntError;
use std::result::Result;

use std::error::Error;

fn get_guess() -> Result<u32, ParseIntError> {
    println!("Enter a guess:");

    let mut guess = String::new();

    stdin().read_line(&mut guess);

    guess.trim().parse::<u32>()
}

fn main() -> Result<(), ParseIntError> {
// fn main() -> Result<(), Box<dyn Error>> {
    let secret_number: u32 = rand::random::<u32>() % 100;

    loop {
        let guess: u32 = get_guess()?;

        if guess > secret_number {
            println!("Guess is higher... Try again!");
        }

        if guess < secret_number {
            println!("Guess is lower... Try again!");
        }

        if guess == secret_number {
            println!("You are correct!");
            break
        }
    };

    Ok(())
}
