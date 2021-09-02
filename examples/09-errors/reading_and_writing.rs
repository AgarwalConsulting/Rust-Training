use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io;

fn read_from_file(f: Result<File, io::Error>) {
    let mut file = match f {
        Ok(file) => file,
        // Ok(mut file) => {
        //     let mut contents = String::new();
        //     let r = file.read_to_string(&mut contents);

        //     println!("Contents: {:?}", contents);
        //     println!("Result: {:?}", r);
        // },
        e => {
            println!("Error encountered: {:?}", e);
            panic!("Error: {:?}", e)
        }
    };

    let mut contents = String::new();
    let r = file.read_to_string(&mut contents);

    println!("Contents: {:?}", contents);
    println!("Result: {:?}", r);
}

fn write_to_file(f: Result<File, io::Error>) -> Result<(), io::Error> {
    // let mut file = f.unwrap();
    let mut file = f?;
    file.write_all(b"Hello, World!").unwrap_or_else( |underlying_err|
        println!("Error: {:?}", underlying_err)
    );

    Ok(())
}

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let f = File::open("/tmp/foo.txt");

    read_from_file(f);

    let f = File::open("/tmp/unicorn");
    // let f = File::create("/tmp/class.txt");

    Ok(write_to_file(f)?)
}
