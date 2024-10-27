use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::env::args;
use std::{thread, time::Duration};

const SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main() {
    let delay = args().nth(1).unwrap_or_default().parse::<u64>().unwrap_or_default();

    println!("server starting on {} with delay: {}", SERVER_ADDRESS, delay);

    let listener = TcpListener::bind(SERVER_ADDRESS).expect("Unable to start server!");

    println!("Server listening on: {}", SERVER_ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, delay);
    }
}

fn handle_connection(mut stream: TcpStream, delay: u64) {
    // Read the buffer
    let mut buffer = [0;1024];
    let len = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("received: {}", message);

    // delay
    thread::sleep(Duration::from_millis(delay));

    let _ = stream.write_all(message.as_bytes());
    println!("written: {}", message);
}
