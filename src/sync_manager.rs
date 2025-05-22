use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Client says: {}", message);

        let response = format!("Received: {}", message);
        stream.write_all(response.as_bytes()).unwrap();
    }
}