use std::option::Option;
use std::fmt::{Display, Debug};

#[derive(Debug)] 
pub struct Graph;           // Concrete type 'Graph'.

pub struct Area<T>(pub T);      // Generic type 'Area'.

#[derive(Debug)] 
pub struct Rectangle(pub Graph); 

pub fn get_value() -> Option<i32> {
    let a: Option<i32> = Some(123456789);
    let b = match a {
        Some(value) => value,
        None => 0,
    };
    Some(b)
}

// Generic function.
pub fn print_value<T: Debug>(option: Option<T>) {
    println!("{:?}", option);
} 

pub fn area<T>(a: Area<T>) -> Area<T> {
    a
}

pub fn generic<T>(g: Area<T>) { }

impl <T> Area<T> where T: Debug {
    pub fn take_area(&self) -> &T {
        &self.0
    }
}

impl Rectangle {
    pub fn output(&self) -> Rectangle {
        Rectangle(Graph)
    }
}
