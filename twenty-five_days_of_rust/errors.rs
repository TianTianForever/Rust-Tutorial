#![crate_type="lib"]
#![crate_name="errors"]

use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

// Create two files with some information.
pub fn setup() {
    File::create("a.txt").and_then(|mut file| file.write_all(b"TianTian"))
                     .unwrap();

    File::create("b.txt").and_then(|mut file|file.write_all(b"Forever"))
                     .unwrap();
}

// Get the data from each file with the data stored in a "Result".
pub fn get_data(path: &str) -> Result<String> {
    File::open(path).map_err(|err| err.to_string())
                    .and_then(|mut file|{
                        let mut contents = String::new();
                        // Read the data into 'contents'.
                        file.read_to_string(&mut contents)
                            .map_err(|err| err.to_string())
                            .map(|_| contents)
                    })
}

// Concat the contents of the two files together into a new 'Result'.
pub fn concat(file1: &str, file2: &str) -> Result<String> {
    let (data1, data2) = (get_data(file1), get_data(file2));
    data1.and_then(|a|
        // Return 'Ok' when both 'a.txt' and 'b.txt' are 'Ok', otherwise
        // return whichever has the first 'Err'.
        data2.and_then(|b| Ok(a + &b))
    )
}

pub fn is_work() {
    println!("work");
}
