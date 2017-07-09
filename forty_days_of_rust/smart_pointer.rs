use std::rc::Rc;
use std::cell::RefCell;
use std::option::Option;

#[derive(Debug)]
enum Kind {
    Value(i32, RefCell<Rc<Kind>>),
    Empty,
}

impl Kind {
    fn tail(&self) -> Option<&RefCell<Rc<Kind>>> {
        match *self {
            Kind::Value(_, ref item) => Some(item),
            Kind::Empty => None,
        }
    }
}

fn main() {
    let a = Rc::new(Kind::Value(23, RefCell::new(Rc::new(Kind::Empty))));           println!("Next item = {:?}", a.tail());
    println!("{:?}", a);
    let b = Rc::new(Kind::Value(24, RefCell::new(a.clone())));
    println!("Next item = {:?}", b.tail());
    println!("{:?}", b);

/*
    if let Some(ref link) = a.tail() { 
        println!("{:?}", *link);      // output: RefCell { value: Empty } 
    }
*/

    if let Some(ref link) = a.tail() {
        // println!("{:?}",*link.borrow_mut() = a.clone());
        *link.borrow_mut() = a.clone();
    }
    
}
