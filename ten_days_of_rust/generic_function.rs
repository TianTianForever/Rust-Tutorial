use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Value<'a, T: Debug + 'a> (&'a T,);

#[derive(Debug)]
struct GenValue<'a, T, U> where
    T: Debug + Display + 'a,
    U: Debug + Display + 'a {
    x: &'a T, 
    y: &'a U,
}


fn main() {
    let x = 1.01;
    let y = 1.02;
    let value = Value(&x);
    let gen_value = GenValue{
        x: &x,
        y: &y,
    };
}
