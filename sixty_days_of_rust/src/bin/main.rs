use std::net::TcpListener;
use std::net::TcpStream;
extern crate sixty_days_of_rust;
use sixty_days_of_rust::ThreadPool;
use std::thread;

fn handle_connection(stream: TcpStream){
    // handle connection code.

}

// Just for test.
struct Contains {
     containers: Vec<Container>
}

impl Contains {
    fn new(size: usize) -> Contains {
        assert!(size > 0);
        let mut containers = Vec::with_capacity(size);
        for id in 0..size {
            containers.push(Container::new(id));
        }
        Contains {
            containers
        }
    }

}

struct Container {
    id: usize,
    threads: thread::JoinHandle<()>,
}

impl Container {
    fn new(id: usize) -> Container {
        let threads = thread::spawn(|| {
            // Handle code.
        });

        Container {
            id,
            threads,
        }
    }
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