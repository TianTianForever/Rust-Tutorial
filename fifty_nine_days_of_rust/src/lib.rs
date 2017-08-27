use std::thread;

/// Create the Thread Pool and Storing Threads.
pub struct ThreadPool {
    //thread: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
}

impl ThreadPool {
     /// Create a new ThreadPool. The size parameter is the number of thread in pool.
     /// The ThreadPool::new use for loop counter to generate an id, creade a new Worker
     /// with that id, and store the worter in the vertor.
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
          let mut workers = Vec::with_capacity(size);
          for id in 0..size {
              // Create some threads and store them in the vector.
              workers.push(Worker::new(id));
          }
          ThreadPool {
              workers: workers,
          }
     }
    pub fn execute<F>(&self, f: F) where 
        F: FnOnce() + Send + 'static
    {
        // Handle code.
    }
}

/// Define a Worker struct that holds an id and a thread::JoinHandle<()>.
pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    /// Define a Worker::new method that take an id number and  return
    /// a Worker istance with that id and a thread.
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {

        });
        Worker {
            id: id,
            thread: thread,
        }
    }
}