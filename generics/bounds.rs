// Define a fuction 'printer' that takes a generic type 'T' which
// must implement trait 'Display'.

use std::fmt::Display;
use std::fmt::Debug;

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

fn pr<T: Display>(p: T) {
   println!("{}",p);
}

trait HasArea {
    fn area(&self) -> f64;
}

struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

fn print_debug<T: Debug>(t: T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: T) -> f64 {
    t.area()
}

fn main() {
    printer("Hello world!");
    pr(1);
    let rectangle = Rectangle{ length: 3.0f64, height: 3.0f64 };
    println!("{}", rectangle.area());
    print_debug(rectangle.area());
    print_debug(area(rectangle));
}
