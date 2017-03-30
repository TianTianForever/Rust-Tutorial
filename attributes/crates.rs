// the crate_type attribute can be used to tell the compiler whether a crate is binary or a library, 
//and the crate_name attribute can be used to set the name of the crate.

// This crate is a library.
#![crate_type = "lib"]

// The library is named 'crates'.
#![crate_name = "crates"]
pub fn time() {
    println!("called crates's 'time()'");
}

pub fn baccktracking() {
    println!("called crates's 'backtracking()'");
}
