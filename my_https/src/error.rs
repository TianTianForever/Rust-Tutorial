use std::error;
use std::fmt;
use std::result;
use Uri;

/// A generic "error" for HTTP connections.
///
#[derive(Debug)]
pub struct Error {
    inner: ErrorKind,
}

pub type Result<T> = result::Result<T, Error>;


// I'll implement 'method request response status' function.
#[derive(Debug)]
enum ErrorKind {
     
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ...
    }
}