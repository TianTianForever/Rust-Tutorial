use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn receiver() {
    let (tx, rx) = mpsc::channel();
    let vector = vec!["tiantian", "forever"];
    
    for item in vector {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(item).unwrap(); 
            thread::sleep(Duration::new(1, 0));
        });
    }
    for receiver in rx {
        println!("{:?}", receiver);
    }
}
fn main() {
    let (tx, rx) = mpsc::channel();
    let mut vector: Vec<i32> = vec![];

    for i in 1..10 {
        let tx = tx.clone();
        thread::spawn( move || {
            tx.send(i).unwrap();
        });
    }
    for _ in 1..10 {
        let receiver = rx.recv().unwrap();
        vector.push(receiver);
        println!("thread spawned: {:?}", receiver);
    }
    //receiver();
}
