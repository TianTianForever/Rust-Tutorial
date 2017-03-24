//executable.rs
//Link to 'library', import items under the 'rary' module.
extern crate rary;
fn main() {
    rary::public_function();
    rary::indirect_access();
}
// Where library.rlib is the path to the compiled library,
// assumed that it's in the same directory here:  

// $rustc --extern rary=library.rlib && ./executable
// called rary's 'public_function()'
// called rary's 'private_functin()'
 
