// rary.rs
// This crate is library
#![crate_type = "lib"]
// When the crate_type attribute is used,we no longer need to pass the --crate-type flag to rustc.
// 
// $rustc rary.rs
// $ls lib*   
// library.rlib

pub fn public_function() {
    println!("called rary's 'public_function'");
}
fn private_function() {
    println!("called rary's 'private_function()'");
}
pub fn indirect_access() {
    print!("called rary's 'private_function()', that \n");
    private_function();
}

