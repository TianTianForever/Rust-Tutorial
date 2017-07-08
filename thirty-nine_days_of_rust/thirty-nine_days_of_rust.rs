use std::cell::Cell;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;

struct Person {
    age: Box<i32>,
    height: Cell<f64>,
    weight: Rc<RefCell<f64>>,
}

impl Deref for Person {
    type Target = Rc<RefCell<f64>>;
    fn deref(&self) -> &Rc<RefCell<f64>> {
        &self.weight
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person {{age: {}, height: {:?}, weight: {:?}}}", 
        self.age, self.height, self.weight)
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {:?}, {:?})", self.age, self.height, self.weight)
    }
}

fn main() {
    let person = Person {
        age: Box::new(23),
        height: Cell::new(170.0),
        weight: Rc::new(RefCell::new(60.0)),
    };
    println!("{:?}", person);
    println!("{}", person);
    assert_eq!(Rc::new(RefCell::new(60.0)), *person);
    person.height.set(170.1);
    println!("{:?}", person);
    *person.borrow_mut() += 10.0;
    println!("{:?}", person);
}

