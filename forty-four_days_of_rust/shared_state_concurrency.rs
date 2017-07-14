use std::thread;
use std::sync;

fn thread1_add_thread2(x: i32, y: i32) {

    let (transmitter, receiver) = sync::mpsc::channel();
    let transmitter2 = transmitter.clone();

    // First thread.
    thread::spawn(move || {
        transmitter.send(x).unwrap();
    });

    // Second thread.
    thread::spawn(move || {
        transmitter2.send(y).unwrap();
    });

    let x = receiver.recv().unwrap();
    let y = receiver.recv().unwrap();

    let z = x + y;
    println!("{:?}", z);
}

fn main() {
    let value = sync::Mutex::new(23);
    {
        let mut number = value.lock().unwrap();
        *number += 1;
        println!("{:?}", number);
    } 
    println!("{:?}", value);
    thread1_add_thread2(1, 2);
    
    let x: Option<i32> = Some(23);
    let y = match x {
        Some(val) => val,
        None => 0,
    };
    assert_eq!(23, y);
}
