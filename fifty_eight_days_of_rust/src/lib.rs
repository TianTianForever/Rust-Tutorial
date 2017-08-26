use std::thread;

/// Create the Thread Pool and Storing Threads.
pub struct ThreadPool {
    thread: Vec<thread::JoinHandle<()>>
}

impl ThreadPool {
     /// Create a new ThreadPool.
     /// The size parameter is the number of thread in pool.
     /// Note that the 'new' function will panic if the size is zero.
     /// # Examples
     ///
     /// Create a thread pool.
     ///
     /// ```
     /// use fifty_eight_days_of_rust::ThreadPool;
     /// let pool = ThreadPool::new(0);  // Panic!!!
     ///
     /// ```
     pub fn new(size: usize) -> ThreadPool {
          assert!(size > 0);
          // The "Vec::new" method will not allocate until elements are pushed  onto it.
          //let mut threads = Vec::new(size);

          // Use 'whith_capacity' method will be able to hode exactly
          // 'capacity' elements without reallocating.
          let mut threads = Vec::with_capacity(size);

          ThreadPool {
              thread: threads
          }
     }
     
    //  pub fn spawn<F, T>(f: F) -> JoinHandle<T> where
    //      F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
    //  {
    //
    //  }
    
    pub fn execute<F>(&self, f: F) where 
        F: FnOnce() + Send + 'static
    {
        // Handle code.
    }
}