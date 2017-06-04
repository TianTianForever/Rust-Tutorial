use std::fmt::Debug;

// Using explicit lifetime annotations for generic type.

// Define generic type 'GenValue'.
#[derive(Debug)]
pub struct GenValue<'a, T: 'a>(pub &'a T);

// Define generic function 'gen_print'.
pub fn gen_print<'a, T>(t: &'a T) where
    T: Debug {
    println!("'gen_print' t is {:?}", t);
}



