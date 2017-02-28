fn main() {
    // Assign a reference of type 'i32', the '&' signifies 
    // there is a reference being assigned.
    let reference = &8;
    match reference {
        // If 'reference's is pattern matched against '&val', it results 
        // in a comparison like:
        // '&i32'
        // '&val'
        // We see that if the matching '&'s are dropped, then the 'i32'
        // should be assigned to 'val'.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the '&', you dereference before matching.
    match *reference {
        val => println!("Got a value via derefencing: {:?}", val), 
    }
    // What if you don't start with a reference? 'reference' was a '&'
    // because the right side was already a reference. this is not a reference
    // because the right side is not one.
    let not_a_reference = 3;

    // Rust provides 'ref' for exactly this purpose. 
    // It modifies the assignment so that a reference is created for the element;
    // this reference is assigned.
    let ref is_a_reference = 3;
    
    // Accordingly, by difining 2 values without references,references
    // can be retrived via 'ref' and 'ref mut'.
    let value = 5;
    let mut mut_value = 6;

    // Use 'ref' keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use 'ref mut' similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can add anything to it.
            *m += 10;
            println!("We added 10, 'mut_value': {:?}", m);       
        
        }    
    }
}

