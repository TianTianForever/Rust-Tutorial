use std::mem;
fn main() {
    // Suffixed literals, their types are known at initialization.
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    // unsuffixed literal, their types depend on how they are used.
    let a = 1;
    let b = 1.1;
   
    // 'size_of_val' returns the size of a variable in bytes.
    println!("size of 'x' in bytes: {}", mem::size_of_val(&x));
    println!("size of 'y' in bytes: {}", mem::size_of_val(&y));
    println!("size of 'z' in bytes: {}", mem::size_of_val(&z));
    println!("size of 'a' in bytes: {}", mem::size_of_val(&a));
    println!("size of 'b' in bytes: {}", mem::size_of_val(&b));
}   
