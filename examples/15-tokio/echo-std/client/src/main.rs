use std::io::prelude::*;
use std::net::TcpStream;

// socat -v tcp-l:1234,fork exec:'/bin/cat'
const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

fn main() {
    println!("connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        let client_addr = stream.local_addr().unwrap();

        println!("connected to echo server {}:{}", client_addr.ip(), client_addr.port());

        let message = "Hello, World!";

        let _ = stream.write(message.as_bytes());
        let _ = stream.flush();

        println!("sent: {}", message);

        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).unwrap();

        let message = String::from_utf8_lossy(&buffer);

        println!("received: {}", message);
    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS);
    }
}
