// A module named "my_module"
mod my_module {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called 'my_module::private_function()'");
    }

    // Use the 'pub' modifier to overide default visibility.
    pub fn function() {
        println!("called 'my_module::function()'");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        println!("called 'my_module::indirect_access()'. that\n>");
        private_function();
    }
    
    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called 'my_module::nested::function()'");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called 'my_module::nested::private_function()'");
        }
    }

    // Nested modules follow the same rules for visibility.
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called 'my_module::private_nested::function()'");
        }
    }
}
fn function() {
    println!("called 'function()'");
}

fn main() {
    function();
    my_module::function();
    my_module::indirect_access();
    my_module::nested::function();
}  
