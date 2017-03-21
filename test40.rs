fn function() {
    println!("super::function()");
}
mod my {
    pub struct Area<T> {
        point: T,
    }
    impl<T> Area<T> {
        pub fn new(point: T) -> Area<T> {
            Area {
                point: point,
            }
        }
    }

    pub fn function() {
        println!("called 'my::function()' ");
    }

    mod cool {
        pub fn function() {
            println!("called 'my::cool::function()'");
        }
       
        // Items can access other items in same module.
        pub fn indirect() {
           
            private_function();
        }
  
        fn private_function() {
            println!("called 'my::cool::private_function()'");
        }
    }
    pub fn indirect_call() {
        // Let's access all the functions named 'function or private_function' from this scope!
        self::function();
        function();
        self::cool::function();
        self::cool::indirect();
        super::function();
        // This will bind to 'cool::function' in the crate scope.
        // In this case the crate scope is the outermost scope.
        {
            use my::cool::function as f;
            f();
        }
    }
}

fn main() {
    my::indirect_call();
    let area = my::Area::new(3.0);
}
