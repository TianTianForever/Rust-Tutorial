// five_days_of_rust.rs

use std::fmt::{Display, Debug};

pub trait MyPrint {
    fn my_print(self);
}

impl<T> MyPrint for T where
    Option<T>: Debug {
    fn my_print(self) {
        println!("{:?}", Some(self));
    }
}

// Define a struct type 'Container'.
pub struct Container(pub i32, pub i32);

// A trait which checks if 2 items are stored inside of 
//container and retrieves first or last value.

pub trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool; // explicitly requires 'A' and 'B'.
   
    // Doesn't explicitly require 'A' or 'B'.
    fn first(&self) -> i32; 
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    // True if the numbers stored are equal.
    fn contains(&self, number_one: &i32, number_two: &i32) -> bool {
        (&self.0 == number_one) && (&self.1 == number_two)
    }
    // Take first number.
    fn first(&self) -> i32 {
        self.0
    }
    // Take last number.
    fn last(&self) -> i32 {
       self.1
    }
}
// Define a function 'deffirence', 'C' contains 'A' and 'B'.
pub fn deffirence<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}
// Generic struct type 'Point'.
pub struct Point<T: Display>(pub T,);

pub trait Value<T> {
    fn take_value(&self) -> &T;
}

impl <T> Value<T> for Point<T> where
    T: Display {
    fn take_value(&self) -> &T {
        &self.0
    }
}
