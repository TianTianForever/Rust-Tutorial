use std::rc::{Rc, Weak};
use std::cell::RefCell;
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let empty: Weak<i64> = Weak::new();
    println!("{:?}", empty.upgrade());
    assert!(empty.upgrade().is_none());     
    // Attemps to upgrade the 'Weak' pointer to an 'Rc', extending
    // the lifetime of the value if successful.
    // Return 'None' if the value has since been dropped.

    let value = Rc::new(23);
    // Rc::strong_count function gets the number of strong(Rc) pointers to this value.
    assert_eq!(1, Rc::strong_count(&value));
    // Create a new 'Weak' pointer to this value.
    let weak_value = Rc::downgrade(&value);
    let strong_value: Option<Rc<_>> = weak_value.upgrade();
    assert!(strong_value.is_some());

    // Destory all strong pointers.
    drop(strong_value);
    drop(value);
    assert!(weak_value.upgrade().is_none());

    let leaf = Rc::new(Node {
        value: 15,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        let branch = Rc::new(Node {
            value: 20,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![leaf.clone()]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("brank strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
       
        println!("leaf strong = {} weak = {}", 
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
