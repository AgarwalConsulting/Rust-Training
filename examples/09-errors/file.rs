use std::fs::File;
use std::io::*;

fn main() {
    let mut f = File::open("/tmp/hello.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("/tmp/unicorn.rs").unwrap_or_else(|create_err| {
                panic!("Unable to create file: {:?}", create_err)
            })
        } else {
            panic!("Unable to open file: {:?}", e);
        }
    });

    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();

    println!("Contents: {}", s);

    // match f {
    //     Ok(f) => println!("{:?}", f),
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => {
    //             match File::create("/tmp/unicorn.rs") {
    //                 Ok(_) => println!("Created file successfully!"),
    //                 Err(create_err) => panic!("Unable to create file: {}", create_err)
    //             };
    //         },
    //         _ => panic!("Unable to open file: {:?}", e)
    //     }
    // }
}
