use std::mem;
// A function which take a closure as an argument and call it.
fn apply<F>(f: F) where 
    // The closure no input and returns nothing.
    F: FnOnce() {
    // ^ TODO: Try changing this to 'Fn'  or 'Fnmut'.
    f();
}

// A function which takes a closure and returns an 'i32'.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an 'i32' and returens an 'i32'.
    F: Fn(i32) -> i32 {
    f(3)
}
fn main() {
    let greeting = "hello";
    // A non-copy type.
    // 'to_owned' creates owned data from borrowed one.
    let mut farewell = "goodbye".to_owned();
    
    // Capture two variables: 'greeting' by reference and 'farewell' by value.
    let diary = || {
        // 'greeting' is by reference: requires 'Fn'.
        println!("I said {}.",greeting);
        
        // Mutation frces 'farewell' to be captured by mutable reference. Now requires 'FnMut'.
        farewell.push_str("!!!!");
        println!("Then i screamed {}.", farewell);
        println!("Now i can sleep. zzzzz");

        // Manually calling drop forces 'farewell' to
        // be captured by value, requires 'FnOne'.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);
    // 'double' satisfies 'apply_to_3' trait bound
    let double = |x| 2 * x;
    println!("three double: {}", apply_to_3(double));
}
