use std::thread;
use std::sync::mpsc;

// Multi-producer, single-consumer FIFO queue communication primitives.
fn receiver() {
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
         let tx = tx.clone();
         thread::spawn(move || {
             tx.send(i).unwrap();
         });
    }
    for _ in 0..10 {
        let re = rx.recv().unwrap();
        println!("{:?}", re);
    }
}

fn main() {
    let execute = thread::spawn(|| {
        for i in 1..11 {
            println!("The number {:?} from the spawned thread", i);
        }
    });

    // execute.join();

    for i in 1..6 {
        println!("The number {:?} from the main thread", i);
    }
    execute.join();

    let mut vector = vec![];
    let child = thread::spawn( move || {
        for i in 1..11 {
            vector.push(i);
            println!("vector: {:?}", vector);
        }
    });
    child.join();

    // Create a simple streaming channel;
    let (tx, rx) = mpsc::channel();   // tx is the sending half and rx is the receiving half.
    thread::spawn(move || {
        tx.send("tiantianforever").unwrap();
    });
   assert_eq!(rx.recv().unwrap(), "tiantianforever");
   receiver();
}  
