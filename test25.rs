<<<<<<< HEAD
//suppress all warning from casts which overflow.
#![allow(overflowing_literals)]
fn main() {
    let decimal = 1.12323f32;
    // Error! no implicit conversion.
    //let integer: u8 = decimal;
    
    // Explicit conversion.
    let integer = decimal as u8;
    let character = integer as char;
    println!("casting: {} -> {} -> {}", decimal, integer, character);
    
    // 1000 -256 - 256 - 256 = 232
    println!(" 1000 as u8 is: {}", 1000 as u8);

    // 1000 
    println!("1000 as u16 is: {}", 1000 as u16);
    
    // -1 + 256 = 255    
    println!("-1 as u8 is: {}", (-1i8) as u8);
   
    
=======
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
>>>>>>> dev
}   
