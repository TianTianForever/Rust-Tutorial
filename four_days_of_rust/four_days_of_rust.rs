use std::fmt::{Debug, Display};
use std::cmp::PartialOrd;

// Define a function 'largest' that takes a generic type 'T'
// Must implement trait 'PartialOrd' and 'Copy'.
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Define a function 'smallest'.
pub fn smallest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut smallest = list[0];
    for &item in list.iter() {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}

// Generic type 'Value'.
pub struct Value<T>(pub T,);

pub trait TakeValue<T> {
     fn take_value(&self) -> &T;
}

impl <T> TakeValue<T> for Value<T> {
     fn take_value(&self) -> &T {
         &self.0
     }
}

// Define a function 'compare_prints' that takes a generic type 'T'
// must implment trait 'Debug' and 'Display'.
pub fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {:?}", t);
}

pub fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

