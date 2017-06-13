#![crate_type="lib"]
#![crate_name="operators"]
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::Add;

#[derive(Clone)]
pub struct User {
   pub email: String,
   pub username: String,
   pub active: bool,
   pub sign_in_count: u64,
}

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.username == other.username;
        self.sign_in_count == other.sign_in_count
    }
}

pub struct Point1<T> where
    T: Debug + Display {
   pub x: T,
   pub y: T,
}
impl<T> fmt::Debug for Point1<T> where
    T: Debug + Display {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point1 {{x: {}, y: {} }}", self.x, self.y)
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Add for Point{
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.y,
            y: self.y + other.y,
        }
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
