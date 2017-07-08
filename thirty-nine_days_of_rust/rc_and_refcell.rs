use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use List::{Cons, Empty};
// Mutiple owners of mutable data by combining Rc<T> and RefCell<T>.

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Empty,
}


struct Value {
    x: Rc<RefCell<i32>>,
    y: Box<i32>,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value {{x: {:?}, y: {} }}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Point {
    x: Rc<Value>,
    y: Option<i32>,
}

fn main() {
    let value = Rc::new(RefCell::new(23));
    println!("Reference count : {}", Rc::strong_count(&value));

    let a = Cons(value.clone(), Rc::new(Empty));
    println!("Reference count after create a: {}", Rc::strong_count(&value));
   
    let share = Rc::new(a);    
    
    let b = Cons(Rc::new(RefCell::new(24)), share.clone());
    let c = Cons(Rc::new(RefCell::new(25)), share.clone());

    *value.borrow_mut() += 1;

    println!("Shared after {:?}", share);
    println!("b after {:?}", b);
    println!("c after {:?}", c);

    let number = Value {
        x: Rc::new(RefCell::new(1992)),
        y: Box::new(1991),
    };
   
    let shared_value = Rc::new(number);
    shared_value.clone();
    let point1 = Point {x: shared_value.clone(), y: Some(2) };
    println!("{:?}", point1);
    let point2 = Point {x: shared_value.clone(), y: None };
    println!("{:?}", point2);
    
}
