fn main() {
    // Declare a variable binding
    let a_binding;
    {
        let x = 2;
        // Initialize the binding.
        a_binding = x * x;
    }
    println!("a_binding: {}", a_binding);
    let another_binding;
    // Error! Use of uninitialized binding.
    // The compiler fobids use of unintialized variables, as this would lead to
    // undefined behavior.
    // println!("another_binding: {}", another_binding);
   
    another_binding = 2;
    println!("another_binding: {}", another_binding);
}
