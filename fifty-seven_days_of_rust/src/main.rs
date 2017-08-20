use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let mut file = File::open("tiantianforever.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        // Responses have this format:
        // HTTP-Version Status-Code Reason-Phrase CRLF
        // headers CRLF
        // message-body
        let response =format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let header = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open("tiantianforever-errors.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let response = format!("{} {}", header, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    //println!("Request: {}",String::from_utf8_lossy(&buffer[..]));
}

fn main() {
    // Create a Tcplistener instance and bind to a port.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established.");
    }
}
