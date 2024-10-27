use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
// use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:8000";

#[tokio::main]
async fn main() {
    println!("connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        let client_addr = stream.local_addr().unwrap();

        println!("connected to echo server {}:{}", client_addr.ip(), client_addr.port());


        let message = "Hello, World!";

        let _ = stream.write_all(message.as_bytes()).await;
        println!("sent: {}", message);

        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).await.unwrap();

        let message = String::from_utf8_lossy(&buffer);

        println!("received: {}", message);
    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS);
    }
}
