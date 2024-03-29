use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let args: env::Args = env::args();

    // let query = &args[1];
    // let filename = &args[2];

    // let (query, filename) = parse_config(&args);

    let config = minigrep::Config::new(args).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}
