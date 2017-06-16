extern crate clones;
use clones::*;
/*
pub trait Clone: ?Size {
    // Return a copy of the value.
    fn clone(&self) -> Self;
    
    // Performs copy-assignment from 'source'.
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
*/

#[derive(Copy)]
struct Stats {
    frequencies: [i32; 100],
}
impl Clone for Stats {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

trait MyClone {
    type Output;
    fn my_clone(&self) -> Self;
}

impl MyClone for Point {
    type Output = Point;
    fn my_clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

fn main() {
    println!("Test");
    let point = Point {x: 123, y: 123};
    let a = point.my_clone();
    println!("{:?}",a);
    let stats = Stats { frequencies: [1; 100] };
    excutable();     
}
