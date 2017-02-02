fn main() {
    // This binding lives in the main function.
    let long_lived_binding = 1;
    
    // This is block, and has a smaller scope than the main function.
    {
        // This binding only exists in this block.
        let short_lived_binding = 2;
        println!(" inner short: {}", short_lived_binding);
        
        // This binding 'shadows' the outer one.       
        let long_lived_binding = 5f32;
        println!("inner long: {}", long_lived_binding);
    }
    // End of the block.



































































































    // Error! 'short_lived_binding' doesn't exists in this scope.
    //println!("Outer short: {}",short_lived_binding);   

    println!("Outer long: {}", long_lived_binding);
    
    // This binding also 'shadow' the previous binding.
    let long_lived_binding = 8u8;
    println!("Outer long: {}", long_lived_binding);
}

