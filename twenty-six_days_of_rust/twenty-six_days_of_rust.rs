use std::io::prelude::*;
use std::fs::File;
use std::io::Read;
type Result<T> = std::result::Result<T, String>;

// Crate two files.
fn setup() -> std::io::Result<()> {
    // Filename 'tiantian.txt'.
    let mut tiantian = try!(File::create("tiantian.txt"));
    try!(tiantian.write_all(
        b" I know how busy you must be and naturally I wouldn't want to take up too much of your time."
     ));    
    // Filename 'forever.txt'.
    let mut forever = try!(File::create("forever.txt"));
    forever.write_all(b"I don't want you to miss it.")
}

// Get the data from each file of 'tiantain.txt' and 'forever.txt'. 
fn get_data(path: &str) -> Result<String> {
    // Try unwraps the value or returns the error.
    let mut file = try!(File::open(path).map_err(|err| err.to_string()));
    let mut contents = String::new();
    // Read date into 'contents'.
    try!(file.read_to_string(&mut contents).map_err(|err| err.to_string()));
    Ok(contents)
}
// Concat the contents of the two files together into a new 'Result'.
fn concat(file_one: &str, file_two: &str) -> Result<String> {
    let (data_a, data_b) = (try!(get_data(file_one)), try!(get_data(file_two)));
    Ok(data_a + &data_b)
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

trait MyRead {
    fn book(&self) -> String;
}

impl MyRead for Point {
    fn book(&self) -> String {
        "TaintainForever".to_string()
    }
}

fn implementation(x: &Point) {
    x.book();
}

fn main() {
    let point = Point {x: 1, y: 2};
    println!("{:?}", point.book());
    implementation(&point);
    setup().unwrap();
    match concat("tiantian.txt", "forever.txt") {
        Ok(o)  => println!("{}", o),
        Err(e) => println!("Error: {}", e),
    }    
}
