use std::thread;
use std::sync::{Mutex, mpsc};
use std::time;
use std::rc::Rc;

fn main() {
    let counter = Mutex::new(0);
//    let mut handles = vec![];
// Error!!!
/*
    // Create ten threads.
    for _ in 1..11 {
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
*/
/*
    let handle = thread::spawn(move || {
         let mut number = counter.lock().unwrap();
         *number += 1;
    });

    handles.push(handle);
    println!("{:?}", handles);
    
    let handle2 = thread::spawn(move || {
        let mut number2 = counter.lock().unwrap();
    });

    handles.push(handle2);
*/
/*
    let counter2 = Rc::new(Mutex::new(0));
    // Create ten threads
    for _ in 1..11 {
        let counter2 = counter2.clone();
        let handle = thread::spawn(move || {
            let number = counter2.lock().unwrap();
            *number += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter2.lock().unwrap());
*/
}
