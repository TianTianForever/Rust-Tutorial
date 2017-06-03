mod my_module;
use my_module::beautiful_gril;
use my_module::beautiful_soup;
use my_module::beautiful_soup::Contains;
fn main() {

    // Called 'my_module::beautiful_soup'.
    let value = beautiful_soup::Value(10.01f64);
    println!("{:?}, {}",value, value.take_value());

    let number_one = 8;
    let number_two = 16;
    let container = beautiful_soup::Container(number_one, number_two);
    println!(
        "Container contains {} and {} is {}",
        &number_one,
        &number_two, 
        container.contains(&number_one, &number_two)
    );

    println!("The difference is {}", beautiful_soup::difference(&container));
    
    
    // Called 'my_module::beautiful_gril::{take_x}'
    let point = beautiful_gril::Point {
        x: 1,
        y: 2,
    };
    println!("The Point of x: {}",point.take_x());
    
    let mut s = String::from("Calculate string length on the heap.");
    { 
        let len = beautiful_gril::calculate_length(&s);
        println!("The length of '{}' is {}",s, len);
    }
    let change_str = beautiful_gril::change(&mut s);
    println!(
        "the length of '{:?}' is {:?}",
        change_str,
        beautiful_gril::calculate_length(&s)
    );

    // Create an immutable Book named 'immutable_book'.
    let immutable_book = beautiful_gril::Book {
        author: "Tom",
        title: "Rust programming language",
        year: 2015,
    };
    let mut mutable_book = immutable_book;
    beautiful_gril::borrow_book(&immutable_book);
    beautiful_gril::borrow_book(&mutable_book);
    beautiful_gril::new_edition(&mut mutable_book);
  

}
