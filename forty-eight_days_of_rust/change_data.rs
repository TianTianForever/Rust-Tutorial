use std::thread;
use std::sync::{RwLock, Arc};
use std::time::Duration;

// There's a difference between asking for a lock in order to read data,
// and in order to write data.

fn read_or_write_data() {
    let data_pointer = Arc::new(
        RwLock::new(
            vec!["some", "useful", "data"]
        )
    );

    let child_data_pointer = data_pointer.clone();
    let child_thread = thread::spawn(move || {
        // Using write method.
        match child_data_pointer.write() {
            Ok(mut data) => {
                data.push("Write data from child thread");
            },
            Err(e) => {
               println!("Failed to get a lock: {}", e);
            }
        };
        
        // Using read method.
        match child_data_pointer.read() {
            Ok(data) => {
                for &line in data.iter() {
                    println!("Child thread: {}", line);
                }
            },
            Err(e) => {
                println!("Failed to get a lock {}", e);
            }
        };
    });
    thread::sleep(Duration::from_millis(1000));
   
    match data_pointer.read() {
        Ok(data) => {
            for &line in data.iter() {
                println!("Read data from parent thread: {}", line);
                thread::sleep(Duration::from_millis(100));
            }
        },
        Err(e) => {
            println!("Failed to get a lock {}", e);
        }
    };

    child_thread.join();
}
fn main() {
    read_or_write_data();   
}
