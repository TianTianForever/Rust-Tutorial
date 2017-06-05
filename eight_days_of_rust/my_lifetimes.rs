use std::string::String;
use std::fmt::{Display, Debug};

// Using explicitly lifetimes annotation.

// Generic type using explicitly lifetimes annotation.
#[derive(Debug)]
pub struct GenValue<'a, T: 'a, U: 'a>(pub &'a T, pub &'a U);


#[derive(Debug)]
pub struct Rectangle<'a, T: 'a> where
    T: Debug + 'a {
    pub height: &'a T,
    pub width: &'a T,
}

#[derive(Clone, Copy)]
pub struct Origin<'a, T: 'a> {
    pub x: &'a T,
    pub y: &'a T,
}

pub struct Point<'a> {
    pub x: &'a u32,
    pub y: &'a u32,
}

// Define generic function 'gen_ref_print'.
pub fn gen_ref_print<'a, T>(t: &'a T) where
    T: Debug + 'a {

    println!("'generic_ref_print': t is {:?}", t);
}
pub fn first_word<'a>(s: &'a String) -> usize {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// Imutable reference
pub fn return_refer_value<'a, 'b>(x: &'a i32, y: &'b i32) -> 
    (&'a i32, &'b i32) {
    (x, y)
}

// Mutable reference
pub fn mut_refer_value<'a>(x: &'a mut i32) {
    *x += 1;
}

// Concrete type 'Empty' 'Null' and 'Double'.
pub struct Empty;
pub struct Null;
pub struct Concrete(pub Empty, pub Null);

// Generic type 'generic'.
pub struct Generic<T>(pub T, pub T);

// Define generic function 'generic_function'.
pub fn generic_function<T>(g: Generic<T>) { }
