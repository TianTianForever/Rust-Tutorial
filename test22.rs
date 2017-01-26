// An attribute to hide warnings for unused code.
#![allow(dead_code)] 

// enum with implicit discriminator (starts at 0).
enum Number {
    Zero,
    One,
    Two,
}

// Enum with implicit discriminator.
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    blue = 0x0000ff,
}
fn main() {
    // 'enums' can be cast as integers.
    println!("zero is {}",Number::Zero as i32);
    println!("one is {}",Number::One as i32);
    println!("red are #{:06x}", Color::Red as i32);
    println!("green are #{:06x}",Color::Green as i32);
 }
