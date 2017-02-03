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
   
    
}   
