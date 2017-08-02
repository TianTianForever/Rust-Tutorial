use std::slice;
// Create a unsafe function or method.
unsafe fn dangerous() {
    // Doing something.
    println!("Using unsafe function or method.");
}
// Error!!!
//fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//    let len = slice.len();
//    assert!(mid <= len);
//    (&mut slice[..mid], &mut slice[mid..])
//}
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    // Use the 'as_mut_ptr' method to get access to the the raw pointer of a slice.
    // Return *mut i32 type.
    let raw_pointer = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (slice::from_raw_parts_mut(raw_pointer, mid),
         slice::from_raw_parts_mut(raw_pointer.offset(mid as isize), len - mid))
    }
}
fn main() {
    let mut number = 23;  // mutable references. 
    let num1 = &number as *const i32;    // immutable raw pointer.
    let num2 = &mut number as *mut i32;  // mutable raw pointer.
    println!("number: {:?}, num1: {:?}, num2: {:?}",number, num1, num2);

    // a raw pointer to an arbitrary memory address.
    let address = 0x1234567;
    let references = &address as *const i32;
    println!("references: {:?}", references);
   
    // Deference raw ponters and read data  by use unsafe block.
    let mut value = 25;
    let value1 = &value as *const i32;
    let value2 = &mut value as *mut i32;
    println!("value: {:?}, 'value1' memory address: {:?} and 'value2' memory address: {:?}",value, value1, value2);
    // Using unsafe block.
    unsafe {
        println!("value1: {}", *value1);
        println!("value2: {}", *value2);
    }
   // Using unsafe function or method by unsafe block.
   unsafe { dangerous(); }
   let mut vector = vec![1, 2, 3, 4, 5, 6];
   let reference_vector = &mut vector[..];
   let (vec1, vec2) = reference_vector.split_at_mut(3);
   println!("vec1: {:?}, vec2: {:?}", vec1, vec2);
}
