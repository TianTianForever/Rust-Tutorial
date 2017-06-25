use std::num::ParseIntError;
use std::fmt;

// type Result<T> = std::result::Result<T, String>;
type Result<T> = std::result::Result<T, DoubleError>;

// Define our error types.
#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element."),
            // This is wrapper so defer to the underlying types' own implementation.
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

fn double_frist(vec: Vec<&str>) -> Result<i32> {
    vec.first()
    // Change the error to our new type.
    .ok_or(DoubleError::EmptyVec)
    .and_then(|v| v.parse::<i32>()
    .map_err(DoubleError::Parse))
    .map(|i| 2 * i)
}

fn my_print(result: Result<i32>) {
    match result {
        Ok(o)  => println!("The first doubled is {}", o),
        Err(e) => println!("Error: {}", e),
    }
}
#[derive(Debug)]
enum Point {
    X,
    Y(ParseIntError),
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Point::X => write!(f, "Don't ignore value"),
            Point::Y(ref e) => e.fmt(f),
        }
    }
}

fn main() {
    let number = vec!["25", "24"];
    my_print(double_frist(number));    
}

