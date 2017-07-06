use std::option::Option;
use std::cell::Cell;
use std::rc::Rc;

/*
pub trait Drop {
    fn drop(&mut self);
}
*/
struct HasDrop {
    x: Option<i32>,
    y: Box<i32>,
    z: Cell<i32>,
}

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
    }
}
struct Inner;
struct Outer(Inner);
impl Drop for Inner {
    fn drop(&mut self) {
        println!("Dropping Inner!"); 
    }
}
impl Drop for Outer {
    fn drop(&mut self) {
        println!("Dropping Outer!");
    }
}
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let hasdrop = HasDrop { x: Some(1), y: Box::new(9), z: Cell::new(93) };
    let x = Outer(Inner);
    let ref_count = Rc::new(2);
    println!("Reference count: {}", Rc::strong_count(&ref_count));    
    use List::{ Cons, Nil };
    let a = Rc::new(Cons(1, Rc::new(Cons(13, Rc::new(Nil)))));
    println!("Reference count after creating b : {}, {:?}", Rc::strong_count(&a), a);
    let b = Cons(3, a.clone());
    println!("Reference count after creating c: {}, {:?}", Rc::strong_count(&a), b);
    let c = Cons(5, a.clone());
    println!("Reference count after goes out of scope: {}, {:?}", Rc::strong_count(&a), c);
}
