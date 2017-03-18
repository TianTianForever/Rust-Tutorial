mod my_module {
    // A public struct with a public filed of generic type 'T'.
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type 'T'.
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }
 
    impl<T> BlackBox<T> {
        // A public constructor method.
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual.
    let white_box = my_module::WhiteBox { 
        contents: "A public struct with a public field of generic type 'T'. " 
    };

    // and their fields can be normally accessed.
    println!("the white box contains: {}", white_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! 'BlackBox' has private fields.
    // let black_box = my_module::BlackBox { 
    //     contents: "classified information" 
    // };

    // However. structs with private fields can be created using public construcors.
    let black_box = my_module::BlackBox::new("classified information");
    
    // and the private fields of a public structs cannot be accessed.
    // Error! the 'contents' fields is private.
    // println!("black_box.contents");
}
