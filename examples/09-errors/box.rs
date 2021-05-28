use std::fs::File;
use std::error::Error;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("/tmp/unicorn_doesn't_exists")?;

    Ok(())
}
