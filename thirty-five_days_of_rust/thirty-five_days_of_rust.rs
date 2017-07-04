mod myerror;
use myerror::*;
use std::ops::Deref;
use std::cell::Cell;

#[derive(Debug)]
struct Rectangle{ height: Cell<f64>, width: f64 }

#[derive(Debug)]
struct Area <'a>{
    rectangle: &'a Rectangle,
}

impl Deref for Rectangle {
    type Target = Cell<f64>;
    fn deref(&self) -> &Cell<f64> {
        &self.height
    }
}

impl<'a> Deref for Area<'a> {
    type Target = Rectangle;
    fn deref(&self) -> &Rectangle {
        &self.rectangle
    }
} 
fn take_height(rectangle: &Rectangle) -> &std::cell::Cell<f64> { 
    &rectangle.height
}

fn main() {
    println!("If you don't traval around, you'd think this is the world.");
    let animal = Animal {
        dog: Box::new(String::from("Dog")),
        cat: String::from("Cat"),
    };
    assert_eq!(Box::new(String::from("Dog")), *animal);

    let vec: Vec<&str> = vec!["24", "25"];
    print_result(mul(vec));

    let rectangle = Rectangle {
        height: Cell::new(6.0),
        width: 5.0,
    };
    let area = Area {
        rectangle: &rectangle,
    };
    let height = take_height(&rectangle);
    println!("{:?}", height);
    println!("{:?}", *area);
}
