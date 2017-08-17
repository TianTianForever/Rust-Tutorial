use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;

// Create 'handle_connection' method and read data from the stream.
fn handle_connection(mut stream: TcpStream) {
    // Declare a 'buffer' on the stack to hold the data that I want to read in. 
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); 
}
fn main() {
    // Listen for TCP connections.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established.");
        handle_connection(stream);
    }
}
