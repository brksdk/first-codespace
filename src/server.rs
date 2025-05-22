use std::net::{TcpListener, TcpStream};
use std::thread;
use crate::sync_manager::handle_client;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind");
    println!("Server running on localhost:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}