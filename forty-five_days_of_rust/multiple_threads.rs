use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::time;

fn multi_threads() {
    let mut counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Create ten threads.
    for _ in 1..11 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut number = counter.lock().unwrap();
            *number += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());  
}

fn transmitter(value: i32) {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(value).unwrap();
    });
    
    let receiver = rx.recv().unwrap();
    println!("Receive message: {:?}", receiver);
}

// Create a rendezvous sync_channel with buffer size 0.
fn message() {
    let (synchronous_sender, receiver) = mpsc::sync_channel(0);
    thread::spawn(move || {
        println!("Sending message...");
        synchronous_sender.send(1).unwrap();
        println!("Message received");
    });
    let received = receiver.recv().unwrap();
    println!("{:?}", received);
}

fn main() {
    multi_threads();
    transmitter(23);
    message(); 
}
