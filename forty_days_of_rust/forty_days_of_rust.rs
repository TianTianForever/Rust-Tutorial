use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initail reference count = {}", Rc::strong_count(&a));
    println!("Next item = {:?}", a.tail());  

    let b = Rc::new(Cons(10, RefCell::new(a.clone())));
    println!("a reference count after crate b = {}", Rc::strong_count(&a));
    println!("b next item = {:?}", b.tail()); 
   
    if let Some(ref link) = a.tail() {
        *link.borrow_mut() = b.clone();
    }

    println!("a reference count after changing a = {}", Rc::strong_count(&b));
    println!("b reference count after changing a = {}", Rc::strong_count(&a));

    //println!("a next item = {:?}", a.tail());
}
