use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;

fn pass_pointer() {
    let data_pointer = Arc::new(
        vec!["tiantian", "forever", "sirius", "matrix"]
    );    
    let child_data_pointer = data_pointer.clone();
    let child_thread = thread::spawn(move || {
        for &line in child_data_pointer.iter() {
            println!("child_thread: {}", line);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    child_thread.join();
}

fn change_data() {
    let data_pointer = Arc::new(
        Mutex::new(
            vec!["some", "userful", "data"]
        )
    );    
    let child_data_pointer = data_pointer.clone();
    let child_thread = thread::spawn(move || {
        match child_data_pointer.lock() {
            Ok(mut data) => {
                data.push("Data from child thread");
            },
            Err(e) => {
                println!("Failed to get a lock: {}", e);
            }
        };
    });
  
    thread::sleep(Duration::from_millis(1000));
    match data_pointer.lock() {
        Ok(data) => {
            for &line in data.iter() {
                println!(" Parent thread: {}", line);
            }
        },
        Err(e) => {
            println!("Failed to get a lock: {}", e);
        }
    }
    child_thread.join();
}

fn main() {
    let outside_value = String::from("TiantianForever");
    let child_thread = thread::spawn(move || {
        println!("{:?} from a main thread.", outside_value);
    });
    pass_pointer();
    change_data();
}
