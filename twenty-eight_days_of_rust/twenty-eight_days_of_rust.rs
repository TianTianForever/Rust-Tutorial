use std::num::ParseIntError;
use std::fmt;
use std::error;
type Result<T> = std::result::Result<T, MulError>;

#[derive(Debug)]
// Multiplication error.
enum MulError {
    EmptyVec,
    Parse(ParseIntError),
}

// Implement the conversion from 'ParseIntError' to 'MulError'.
// This will be automatically called by 'try!' if a 'ParseIntError' needs
// converting into a 'MulError'.
/*
pub trait From<T> {
    fn from(T) -> Self;
}
*/
impl From<ParseIntError> for MulError {
    fn from(err: ParseIntError) -> MulError {
        MulError::Parse(err)
    }
}
impl fmt::Display for MulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MulError::EmptyVec  =>
                write!(f,"Please use a vector with at least one element."),
            MulError::Parse(ref e) => e.fmt(f),
        }
    }
}
fn mul_first(vec: Vec<&str>) -> Result<i32> {
    let first = try!(vec.first().ok_or(MulError::EmptyVec));
    let parsed = try!(first.parse::<i32>());
    Ok( 2 * parsed)
}
fn my_print(result: Result<i32>) {
    match result {
        Ok(o)   => println!("the first double is {}", o),
        Err(e)  => println!("Error: {}", e),
    }
}
fn main() {
    println!("Progress is activity of today and the assurance tomorrow.");
    let empty = vec![];
    my_print(mul_first(empty));
    let vector = vec!["12", "13", "24", "tiantian", "forever"];
    my_print(mul_first(vector));
}


