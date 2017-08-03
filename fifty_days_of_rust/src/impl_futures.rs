extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;
use std::time::Duration;
use futures::Future;
use futures_cpupool::CpuPool;
use self::tokio_timer::Timer;

// Synchronous version.
pub fn synchrony(number: u64) -> bool {
    for i in 2..number {
        if number % i == 0 {
            return false
        }
    }
    true
}

// Asynchronous version.
pub fn asynchrony(number: u64) {
    // set up a thread pool.
    let pool = CpuPool::new_num_cpus();
    // spawn our computation, getting back a 'future' of the answer.
    let prime_future = pool.spawn_fn(move || {
        let prime = synchrony(number);
        let res: Result<bool, ()> = Ok(prime);
        res
    });
    println!("Create Asynchronous versin.");
    if prime_future.wait().unwrap() {
        println!("Prime");
    } else {
        println!("Not prime");
    }
}

pub fn asynchrony_v2(number: u64) {
    let pool = CpuPool::new_num_cpus();
    let timer = Timer::default();
    let timeout = timer.sleep(Duration::from_millis(1000)).then(|_| Err(()));
    let prime = pool.spawn_fn(move || {
        Ok(synchrony(number))
    });
    let selection = timeout.select(prime).map(|(win, _)| win);
    //let selection = timeout.select(prime).map(|(s, _)| s);
    match selection.wait() {
        Ok(true) => println!("Prime"),
        Ok(false) => println!("Not prime"),
        Err(_)  => println!("asynchronous version: Timed out."),
    }
}
