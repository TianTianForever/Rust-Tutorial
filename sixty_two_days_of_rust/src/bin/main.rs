extern crate sixty_two_days_of_rust;
use sixty_two_days_of_rust::ThreadPool;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::prelude::*;
use std::fs::*;
fn handle_connection(mut stream: TcpStream){
    // handle connection code.
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();
    //println!("request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "doc/sixty_two_days_of_rust/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "doc/sixty_two_days_of_rust/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let response = format!("{} {}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// Just for test.
type Job = Box<FnOnce() + Send + 'static>;
struct Contains {
     containers: Vec<Container>,
     sender: mpsc::Sender<Job>,
}
// Send message.
impl Contains {
    fn new(size: usize) -> Contains {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut containers = Vec::with_capacity(size);
        for id in 0..size {
            containers.push(Container::new(id, receiver.clone()));
        }

        Contains {
            containers,
            sender,
        }
    }
    fn exe<F>(&self, f: F) where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
struct Container {
    id: usize,
    threads: thread::JoinHandle<()>,
}
// Receive message.
impl Container {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Container {
        let threads = thread::spawn(move || {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("The Container {} got a job, executate", id);
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
    let mut counter = 0;

    for stream in listener.incoming() {
        if counter == 2 {
            println!("Shutting down.");
            break;
        }
        counter +=1;

        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}