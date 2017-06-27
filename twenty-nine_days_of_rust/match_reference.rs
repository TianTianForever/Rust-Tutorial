use std::num::ParseIntError;
use std::fmt;
// 'ref' keyword creates a reference for use in the pattern.
/// ```
/// let x = 5;
/// match x {
///
///     ref r => println("Got the value: {}", r),
///
/// }
///
/// ```

type Result<T> = std::result::Result<T, MyError>;
#[derive(Debug)]
enum MyError {
    ParseError,
    Parse(ParseIntError),
}
// Implement the conversion from "ParseIntError" to "MyError".
impl From<ParseIntError> for MyError {
    fn from(error: ParseIntError) -> MyError {
        MyError::Parse(error)
    }
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::ParseError => write!(f, "Parse error"),
            MyError::Parse(ref e) => e.fmt(f),
        }
    }
}

fn main() {

}
