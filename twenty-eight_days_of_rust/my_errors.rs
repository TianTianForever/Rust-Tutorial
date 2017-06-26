use std::num::ParseIntError;
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<error::Error>>;
/*
pub trait Error: Debug + Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}
*/
//type Result<T> = std::result::Result<T, MyError>;
#[derive(Debug)]
enum MyError {
    EmptyValue,
    Parse(ParseIntError),
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> MyError {
        MyError::Parse(err)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::EmptyValue   => 
                write!(f, "Empty value"),
            MyError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for MyError {
    fn description(&self) -> &str {
        match *self {
            MyError::EmptyValue => "Empty vectors not allowed",
            MyError::Parse(ref e) => e.description(),
        }
    }
   
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            MyError::EmptyValue => None,
            MyError::Parse(ref e) => Some(e),  
        }
    }
}

fn mul_first(vec: Vec<&str>) -> Result<i32> {
    let first = try!(vec.first().ok_or(MyError::EmptyValue));
    let parsed = try!(first.parse::<i32>());
    Ok(2 * parsed)
}

fn my_print(result: Result<i32>) {
    match result {
        Ok(o)  => println!("Result: {}", o),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let empty = vec![];
    my_print(mul_first(empty));
}
