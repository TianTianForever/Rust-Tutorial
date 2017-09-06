#![feature(conservative_impl_trait)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Job;  
pub enum Message<'a> {
    execute(&'a Job),
    error,
}
/// An interface for dealing with  files.
pub trait OperateFile {
     /// Read the contens of a file into a `String`.
     fn read_into_string(&self, file: &str) -> std::io::Result<()>;
     /// Create a new file and write byte to it.
     fn create_file_and_write(&self, file_name: &str, contents: &[u8]) -> std::io::Result<()>;
     ///Using 'BuffReader' can be efficient to read the contents of a file.
     fn buffer_read(&self, file_name: &str, contents: &[u8]) -> std::io::Result<()>;
}

/// Get a even values in range.
pub fn get_even_valuse_in_range<T: IntoIterator<Item = i32>>(range: T) 
    -> impl Iterator<Item = i32> {
    range.into_iter().filter(|x| x & 1 == 0)    
}

pub trait Int32Iterator: Iterator<Item = i32> { }
impl <T: Iterator<Item = i32>> Int32Iterator for T { }
pub fn get_even_valuse_in_range_one<T: IntoIterator<Item = i32>>(range: T)
    -> impl Int32Iterator {
        range.into_iter().filter(|x| x & 1 == 0)
}

pub trait IntoInt32Iterator: IntoIterator<Item = i32> { }
impl <T: IntoInt32Iterator<Item = i32>> IntoInt32Iterator for T { }
pub fn get_even_valuse_in_range_two<T: IntoInt32Iterator<Item = i32>>(range: T) 
    -> impl Int32Iterator {
        range.into_iter().filter(|x| x & 1 == 0)
    }

impl OperateFile for Job {
     fn read_into_string(&self, file: &str) -> std::io::Result<()> {
         let mut file = File::open(file)?;
         let mut contents = String::new();
         file.read_to_string(&mut contents)?;
         Ok(())
     }
     fn create_file_and_write(&self, file_name: &str, contents: &[u8]) -> std::io::Result<()> {
         let mut file = File::create(file_name)?;
         file.write_all(contents)?;
         Ok(())
     }
     fn buffer_read(&self, file_name: &str, contents: &[u8]) -> std::io::Result<()> {
         let file = File::open(file_name)?;
         let mut buf_read = BufReader::new(file);
         let mut contents = String::new();
         buf_read.read_to_string(&mut contents)?;
         Ok(())
     }
}
