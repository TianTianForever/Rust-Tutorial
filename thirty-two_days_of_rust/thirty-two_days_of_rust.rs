mod genericity;
use genericity as gen;
use std::option::Option;
use std::fmt::Debug;
use std::result::Result;

// Error
/*
#[derive(Debug)]
enum MatchError {
    EmptyValue,
}
type MatchResult<T> = std::result::Result<T, MatchError>;

fn parse(vec: Vec<&str>) -> MatchResult<i32> {
    if vec == ' ' {
        Err(MatchError::EmptyValue)
    } else {
        let value = try!(vec.first());
        Ok(value.parse::<i32>())
    }
}
*/

fn main() {
    println!("Care and diligent bring luck.");

    println!("{:?}", gen::take_value(None));
    println!("{:?}", gen::take_value(Some(1993)));

    gen::div(1.1, 1.1);
    println!("{:?}", gen::square_root(2.0));
    println!("{:?}", gen::ln(10.0));
    gen::operate(1.0, 1.0);
    //gen::operate(0.0, 0.0);  // Panic!!!
    
}
