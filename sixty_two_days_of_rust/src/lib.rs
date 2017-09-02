use std::thread;
use std::sync::{mpsc, Arc, Mutex};

/// This `Message` enum will either be a `NewJob` variant that holds the job
/// the thread should run, or it will be a `End`variant that will cause the 
/// thread to exit loop and stop.
pub enum Message {
    NewJob(Job),
    End,
}

/// Create the Thread Pool and Storing Threads. It will create a channel and 
/// hold on to the sending side.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

/// The version of the call operater that takes a by value receiver.                                                               
/// The `FnBox` trait to work around the  limitaions of `Box<FnOnce()>`.
pub trait FnBox {
    fn call_box(self: Box<Self>);
}
impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
/// Define a Job type alias that it will hold the closures we want to send down the channel.
pub type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
     /// Create a new `ThreadPool`. The size parameter is the number of thread in pool.
     /// The `ThreadPool::new` use for loop counter to generate an id, creade a new `Worker`
     /// with that `id`, and store the worter in the vertor.
     /// Note that the 'new' function will panic if the size is zero.
     /// # Examples
     ///
     /// Create a thread pool.
     ///
     /// ```
     /// use fifty_nine_days_of_rust::ThreadPool;
     /// let pool = ThreadPool::new(0);     // Panic!!!
     /// let worker = ThreadPool::new(5);   // Created successfully.
     /// ```
     pub fn new(size: usize) -> ThreadPool {
          assert!(size > 0);
          // The "Vec::new" method will not allocate until elements are pushed  onto it.        
          // Use 'whith_capacity' method will be able to hode exactly
          // 'capacity' elements without reallocating.
          let (sender, receiver) = mpsc::channel();
          let receiver = Arc::new(Mutex::new(receiver));
          let mut workers = Vec::with_capacity(size);
          for id in 0..size {
              // Create some threads and store them in the vector.
              workers.push(Worker::new(id, receiver.clone()));
          }

          ThreadPool {
              workers: workers,
              sender: sender,
          }
     }
    pub fn execute<F>(&self, f: F) where 
        F: FnOnce() + Send + 'static
    {
         let job = Box::new(f);
         self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending end message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::End).unwrap();
        }
        println!("Shutting down worker");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
/// Define a Worker struct that holds an `id` and a `thread::JoinHandle<()>`.
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    /// Define a `Worker::new` method that take an `id` number and  return
    /// a `Worker` istance with that id and a thread.
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} get a job, executing.", id);
                        job.call_box();
                    },
                    Message::End => {
                        println!("Worker {}  end.", id);
                        break;
                    },
                }
            }
        });
        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
