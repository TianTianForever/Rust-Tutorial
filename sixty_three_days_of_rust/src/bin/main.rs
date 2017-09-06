extern crate sixty_three_days_of_rust;
use sixty_three_days_of_rust::*;
use std::io;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let job = Job;
    let _execute = Message::execute(&job);
    let _error = Message::error;
    
    let mut file = "tiantian.txt";
    job.read_into_string(&file);

    let mut file_name = "forever.txt";
    let contents = b"Create a new file and write byte to it.";
    job.create_file_and_write(file_name, contents);

    let even_values = get_even_valuse_in_range(1..5);
    for value in even_values {
        println!("{}", value);
    }
    // print_even_values(even_values);
}

// fn print_even_values<T: Int32Iterator + IntoInt32Iterator>(even_values: T) {
//     for value in even_values {
//         println!("{}", value);
//     }
// }
