use std::any::Any;
/*
// A pointer type for heap allocation.
pub struct Box<T: ?Sized>(Unique<T>);

impl<T> Box<T> {
    // Allocates memory on the heap and then places x into it.
    // Note that this doesn't actually allocate if T is zero-sized.
    pub fn new(x: T) -> Box<T> {
        box x
    }
}

impl<T: ?Size> Box<T> {
    // Constructs a box from a row pointer.
    // 'Box::into_raw'
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        mem::transmute(raw)
    }

   // Consumes the 'Box', returning the wrapped raw pointer.
   fn into_raw(b: Box<T>) -> *mut T {
       unsafe{ mem:: transmute(b) }
   }
}

impl Box<Any> {
    // Attempt to downcast the box to a concrete type.
    pub fn downcast<T: Any>(self) -> Result<Box<T>, Box<Any>> {
        if self.is::<T>() {
            unsafe {
                let raw: *mut Any = Box::into_raw(self);
                OK(Box::from_raw(raw as *mut T))
            }
        } else {
            Err(self)
        }
    }
}
*/
fn if_is_string(value: Box<Any>) {
    if let Ok(string) = value.downcast::<String>() {
        println!("String({}) bytes -> {}", string.len(), string);
    } else {
        println!("Not a string...");
    }
}
fn main() {
    let eighteen = Box::new(18);         // Allocates memory on the heap.
    let ptr = Box::into_raw(eighteen);   // consumes the 'Box::new(18)', returning the wrapped raw pointer.
    let x = unsafe {Box::from_raw(ptr)}; // Constructs a box 'x' from a raw poiter.
    let my_string = "The eighteenth day of rust".to_string();
    if_is_string(Box::new(my_string));
    if_is_string(Box::new(0i16));
}
