use std::thread;
use std::sync::{mpsc, Arc, Mutex};
#[derive(Debug)]
struct AveragedCollection {
    list: Vec<i32>,
    average: f64, 
}

impl AveragedCollection {
    fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }
    fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    } 
}
fn calculate() {
    let number = Arc::new(Mutex::new(0));
    let mut vector = vec![];
    for _ in 0..10000 {
        let number = number.clone();
        let handle = thread::spawn(move || {
            let mut num = number.lock().unwrap();
            *num += 1;
        });
        vector.push(handle);
    }
    for vec in vector {
        vec.join().unwrap();
    }
    println!("{:?}", *number.lock().unwrap());
}
fn main() {
    calculate();
    let mut list = vec![23, 24, 25, 26];
    let mut average = AveragedCollection {
        list: list,
        average: 0.0,
    };
    average.add(27);
    println!("{:?}", average);
}
