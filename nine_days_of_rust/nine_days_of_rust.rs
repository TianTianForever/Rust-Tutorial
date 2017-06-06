extern crate lifetimes;
mod lib;
use lib::associated_types as associated;
use associated::{Point, CollectionValue};
use lib::manually_implement as manually;
use lifetimes::*;

fn main() {

    let string = String::from("This 'String::from' create on the heap.");

    // '&str' type is immutable reference.
    let str_stack = "Create it on the stack.";

    let word = take_word(&string);
    println!("{:?}",word);
    assert_eq!("This", word); 

    let word2 = take_word(&str_stack);
    println!("{:?}", word2);

    // Using module 'lib::associate_types'.
    let x = 12345;
    let y = 54321;
    let position = associated::Position {
        x: x,
        y: y,
    };
    assert_eq!(12345, position.position_x());   
    println!("{:?}",position.position_y());
   
    println!(
        "using associated types: {:?} - {:?} = {:?}",
        position.position_y(),
        position.position_x(),
        associated::using_associated_types(&position)
    );
    assert_eq!(true,position.point(&x, &y));
 
    let my_value = associated::Value(1234567890.0987654321, 9876543210.123456789);
    assert_eq!(1234567890.0987654321, my_value.take_value_one());
    assert_eq!(9876543210.123456789, my_value.take_value_two());   
    println!(
        "Uisng associated types: {:?} - {:?} = {:?}",
        my_value.take_value_one(),
        my_value.take_value_two(),
        associated::collection(&my_value)
    ); 
   
    // Using module 'lib::manually_implement' module.
    let user = manually::User {
        email: String::from("someone@example.com"),
        username: String::from("someoneuser666"),
        active: true,
        sign_in_count: 2,
    };
    println!("{:?}",user);
    println!("{}",user)
}
