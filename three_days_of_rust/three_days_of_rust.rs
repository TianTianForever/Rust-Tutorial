// three_days_of_rust.rs

// This crate is library.
// Generic type 'Point'.
pub struct Point<T>(pub T, pub T);

// implement of Point for a generic type 'T'.
impl <T> Point<T> {
    
    pub fn take_x(&self) -> &T {
        &self.0         
    }

    pub fn take_y(&self) -> &T {
        &self.1
    }      
}

// Generic type 'Value'.
pub struct Value<T>(pub T);

pub struct Rectangle {
    pub length: f64,
    pub width: f64,
}

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

pub trait HasArea {
   fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        &self.length * &self.width
    }
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

// Define a generic function 'print_area'.
pub fn print_area<T: HasArea> (shape: T) {
    println!("This shape has an area of {}", shape.area());
} 




