// Link to library,import items from 'five_days_of_rust' module.
extern crate five_days_of_rust as gl;
use gl::{Value, MyPrint, Contains};

fn main() {
    let point = gl::Point(1.0f64);
    //point.take_value();
    println!("{:?}",point.take_value());
    let vec = vec![1, 5, 7, 9];
    vec.my_print();
    
    let number_one = 1;
    let number_two = 2;
    let container = gl::Container(number_one, number_two);
    println!(
        "Does container contains {} and {} : {}",
        &number_one,
        &number_two,
        container.contains(&number_one, &number_two),
    );
   
    println!(
        "First number: {}, Last number: {}",
         container.first(),
         container.last(),
    );    

    println!("The deffirence is: {}", gl::deffirence(&container));
}
