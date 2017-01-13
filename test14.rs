use std::fmt;

// Define a structure which 'fmt::Display' will be implemented for. 
// This is simply a tuple struct containing an 'i32' bound to the name 'Point'.
struct Point(i32);

//In order to use the '{}' marker, the trait 'fmt::Display' must be implemented manually for the type.
impl fmt::Display for Point {
    //This trait requires 'fmt' with this exact signature.
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{}",self.0)
     }
}

#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}
//Similarly, implement 'Dispaly' for 'Point2'.
impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}",self.x, self.y)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement 'Display' for 'MinMax'.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})",self.0, self.1)
    }
}
fn main() {
    println!("{}",Point(1));

    println!("Compare structures:");
    let minmax = MinMax(11, 25);
    println!("Display: {}",minmax);
    println!("Debug: {}",minmax);

    let point2 = Point2 {x: 1.1, y: 1.2};
    println!("display: {}",point2);
    println!("debug: {:?}",point2);
}
