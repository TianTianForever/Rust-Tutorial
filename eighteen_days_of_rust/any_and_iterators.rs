use std::iter::Iterator;
use std::option::Option;
use std::any::{Any, TypeId};
/*
pub trait Iterator {
    // the type of the elements being iterated over.
    type Item;
   fn next(&mut self) -> Option<Self::Item>;
}
*/
struct Sequence {
    current: u32,
    next: u32,
}

impl Iterator for Sequence {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;
        Some(self.current)
    }
}
/*
pub trait Any: 'static {
    // Gets the 'TypeId' of 'self'.
    fn get_type_id(&self) -> TypeId;
}
impl <T: 'static + ?Size> Any for T {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}
impl Any {
    pub is<T: Any>(&self) -> bool {
    // Get TypeId of the type this function is instantiated with
    let t = TypeId::of::<T>();

    // Get TypeId of the type in the trait object.
    let boxed = self.get_type_id();
  
    t == boxed
    }
}
*/
fn is_string(s: &Any) {
    if s.is::<String>() {
        println!("It's a string");
    } else {
        println!("Not a string");
    }
}
fn main() {
    let a = [2, 4, 6];
    let mut iter = a.iter();
    // A call to next() returns the next value.
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&4), iter.next());
    assert_eq!(Some(&6), iter.next());
    assert_eq!(None, iter.next()); 
    let mut s = 0..10;
    for i in 0..11 {
        println!("{:?}", s.next());
    }
    is_string(&1111111110);
    is_string(&"eighteenth".to_string());
}
