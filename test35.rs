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

    let not_a_reference = 3;
    let ref is_a_reference = 3;
    
}
