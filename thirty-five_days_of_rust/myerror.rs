use std::num::ParseIntError;
use std::ops::Deref;
use std::fmt;

#[derive(Debug)]
pub struct Animal {
    pub dog: Box<String>,
    pub cat: String,
}

impl Deref for Animal {
    type Target = Box<String>;
    fn deref(&self) -> &Box<String> {
        &self.dog
    }
}

type Result<T> = ::std::result::Result<T, MyError>;
pub enum MyError {
    EmptyValue,
    Parse(ParseIntError),
}

impl From<ParseIntError> for MyError {
    fn from(error: ParseIntError) -> MyError {
        MyError::Parse(error)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::EmptyValue =>
                write!(f, "Empty value"),
            MyError::Parse(ref e) => e.fmt(f),
        }
    }
}

pub fn mul(vec: Vec<&str>) -> Result<i32> {
    let value = try!(vec.first().ok_or(MyError::EmptyValue));
    let parsed = try!(value.parse::<i32>());
    Ok(parsed * parsed)
}
pub fn print_result(result: Result<i32>) {
    match result {
        Ok(ok) => println!("Result: 24 * 24 = {}", ok),
        Err(error) => println!("Error: {}", error),
    }
}
