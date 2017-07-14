use std::thread;
use std::sync::mpsc;
use std::time;

fn received() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let values = vec!["Tiantian", "forever"];
        for value in values {
           tx.send(value).unwrap();
           thread::sleep(time::Duration::new(1, 0));
        }
    });

    for received in rx {
        println!("Got: {:?}", received);
    }
}

fn multi_producers() {
    let(tx, rx) = mpsc::channel();
    // Multiple producers by cloning the transmitter.
    let tx1 = tx.clone();
    thread::spawn(move || {
        let username = vec!["tiantian", "forever"];
        for content in username {
            tx.send(content).unwrap();
            thread::sleep(time::Duration::new(1, 0));
        }
/*
        for received in rx {
            println!("Thread1: {:?}", rx);
        }
*/
    });
    thread::spawn(move || {
        let email = vec!["example@someone.com", "username@example.com"];
        for content in email {
            tx1.send(content).unwrap();
            thread::sleep(time::Duration::new(1, 0));
        }
/*
        for receiver in rx {
            println!("thread2: {:?}", receiver);
        }
*/
    });

        for receiver in rx {
            println!("{:?}", receiver);
        }
}

fn main() {
    received();
    multi_producers();
}
