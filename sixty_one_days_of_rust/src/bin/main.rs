use std::net::TcpListener;
use std::net::TcpStream;
extern crate sixty_one_days_of_rust;
use sixty_one_days_of_rust::ThreadPool;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

fn handle_connection(stream: TcpStream){
    // handle connection code.
}

// Just for test.
// Holds a closures.
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
            let Job = receiver.lock().unwrap().recv().unwrap();
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

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}