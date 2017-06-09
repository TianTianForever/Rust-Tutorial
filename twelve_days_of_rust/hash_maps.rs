#![crate_type = "lib"]
#![crate_name = "hash_maps"]
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    // Create a new Point.
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

