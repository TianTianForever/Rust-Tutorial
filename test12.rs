//standard output

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
fn main() {

   //Create a path to the desired file
   let path = Path::new("hello.txt");
   let display = path.display();
   
   // Open the path in read-only mode, return the io::Result<File> types.
   let mut file = match File::open(&path) {
       // The 'description' method of 'io::Error' return a string that
       // describes the error
       Err(why) => panic!("Couldn't open {}: {}",display, why.description()),
       Ok(file) => file, 
   };
   
   //Read the file contents into a string, return the io::Result<usize> types.
   let mut s = String::new();
   match file.read_to_string(&mut s) {
       Err(why) => panic!("Couldn't read {}: {}",display, why.description()),
       Ok(_) => println!("{} contains: \n{}",display, s),
   }

}

