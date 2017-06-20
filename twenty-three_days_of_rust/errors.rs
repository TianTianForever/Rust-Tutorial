#![crate_type="lib"]
#![crate_name="errors"]
use std::result;
use std::num::ParseIntError;

#[derive(Debug)] pub enum Point { X, Y, Z }
#[derive(Debug)] pub enum Number { One, Two, Empty }

pub fn have_x_and_y(point: Point) -> Option<Point> {
    match point {
        Point::Z => None,
        _   => Some(point),
    }

}

pub fn take_x_and_y(point: Point) -> Option<Point> {
    match have_x_and_y(point) {
        None => None,
        Some(point) => Some(point),
    }

}

pub fn print_x_and_y(point: Point, number: Number) {
    match have_x_and_y(point) {
        None => println!("Z : {:?}", number),
        Some(point) => println!("{:?} : {:?}", point, number),
    }
}
// A generic alias for any 'Result' with this specific 'Err' type.
// type Result<T> = result::Result<T, ParseIntError>;


pub fn parse_number(string: &str) -> Result<i32, ParseIntError> {
    match string.parse::<i32>() {
        Ok(s)  => Ok(s),
        Err(e) => Err(e),
    }
}

/*
pub fn gen_parse_number(string: &str) -> Result<i32> {
    match string.parse::<i32>() {
        Ok(o)  => Ok(o),
        Err(e) => Err(e),
    }
}
*/

// Output result.
pub fn output(result: Result<i32, ParseIntError>) {
    match result {
        Ok(r)  => println!("Result: {:?}", r),
        Err(e) => println!("Error result: {:?} !!!", e),
    }
}
