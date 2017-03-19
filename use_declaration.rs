// Bind the " my_module::nested::function" path to "my_function".
use my_module::nested::function as my_function;

fn function() {
    println!("called 'function'.");
}
mod my_module {

    pub mod nested {
        pub fn function() {
            println!("called 'my_module::nested::function()'");
        }
    }
}

fn main() {
    function();

    // easier access to "my_module::nested::function".
    my_function();    

    println!("Entering block");
    {
        // This is equivalent to 'use my_module::nested::function as function'.
        // This 'function()' will shadow the outer one.
        use my_module::nested::function;
        function();

        // 'use' bindings have a local scope. In this case, the
        // shadowing of 'function()' is only in this block.
        println!("leaving block");
    }
}
