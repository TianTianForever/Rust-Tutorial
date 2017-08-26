use std::net::TcpListener;
use std::net::TcpStream;
extern crate fifty_eight_days_of_rust;
use fifty_eight_days_of_rust::ThreadPool;

fn handle_connection(stream: TcpStream){
    // handle connection code.

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Create a thread pool with a configurable numble of threads.
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}