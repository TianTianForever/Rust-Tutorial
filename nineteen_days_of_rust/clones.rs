#![crate_type="lib"]
#![crate_name="clones"]

// A unit struct without resources
#[derive(Debug, Clone, Copy)]
pub struct Zero;

#[derive(Debug,Clone)]
pub struct Value(Box<usize>, Box<usize>);

pub fn excutable() {
    let zero = Zero;                  // Instantiate 'Zero'.
    let copy_zero = zero;             // Copy 'Zero'.
    
    // Both 'Zero' can be used independently.
    println!("original: {:?}", zero);
    println!("copy: {:?}", copy_zero);

    let value = Value(Box::new(19), Box::new(92));
    println!("original: {:?}", value);

    let copy_value = value;            // Copy 'value' into copy_value, moves resources
    println!("copy: {:?}", copy_value);

    //println!("re_original: {:?}", value);  // Error
   
}
