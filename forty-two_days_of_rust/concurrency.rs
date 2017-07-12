use std::thread;
use std::sync::mpsc;

fn main() {
    
    let child = thread::spawn(|| {
        for i in 1..10 {
            println!("The number {} from the spawned thread!!!", i);
        }
    });
    
    child.join();
 
    for i in 1..3 {
        println!("The number {} from the main thread!!!", i);
    } 
    
    let vector = vec![1, 2, 3, 4];
    let handle = thread::spawn(move || {
         println!("vector: {:?}", vector);
    });

    handle.join();
}
