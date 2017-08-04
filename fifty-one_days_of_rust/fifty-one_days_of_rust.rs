use std::option::Option;
use std::result::Result;

fn take_value(value: Option<i32>) -> Result<i32, ()> {
    match value {
        Some(x) => Ok(x),
        None => Err(()),
    }
}

// FFI
extern "C" {
    fn calculate_value(a: f64, b: f64) -> f64;
}

fn add_value(a: f64, b: f64) -> f64 {
    a + b
}

fn test() {
    let x = Some(3);
    println!("{:?}", take_value(x));

    fn calculate_value(a: f64, b: f64) -> f64 {
        a + b
    }
    assert_eq!(4.0, calculate_value(2.0, 2.0));
    assert_eq!(4.0, add_value(2.0, 2.0));  
    
    // Using 'ref' keyword.
    let str1 = Some(String::from("tiantian"));
    match str1  {
        Some(ref name) => println!("Found a name: {:?}", name),  // Create a reference.
        None => (),
    } 

    // Using ' ref mut' keyword.
    let mut str2 = Some(String::from("forever"));
    match str2 {
        Some(ref mut name) => *name = String::from("tiantian forever"),
        None => (),
    }
    println!("{:?}", str2);
}

struct Tree {
    node: Box<String>,
    branch: Box<i32>,
}

fn main() {
    test();
    let tree = Tree {
        node: Box::new(String::from("Node")), 
        branch: Box::new(23)
    };
    match tree {
        Tree { node, .. } => println!("node is {}", node),
    }
}
