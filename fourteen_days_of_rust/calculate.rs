use std::ops::{ Mul, Div};
#[derive(Debug)]
struct Rectangle<T> {
    height: T,
    weight: T,
}

/*
trait Mul<RHS=Self> {
    // The resulting type after applying the '*' operator.
    type Output;

    // The method for the '*' operator.
    fn mul(self, rhs: RHS) -> Self::Output;
}

*/
impl<T> Rectangle<T> {
    fn new(height: T, weight: T) -> Rectangle<T> {
        Rectangle {
            height: height,
            weight: weight,
        }
    }
}

/*
impl <T> Mul<T> for Rectangle<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Rectangle::new(self.height * rhs.height, self.weight * rhs.weight)  
    }
}
*/

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point {
            x: x,
            y: y,
        }
    }
}

// Implement Multiplication.
impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
    Point::new(self.x * rhs.x, self.y * rhs.y)
    }
}
fn main() {
   let point = Point::new(1, 2); 
   println!("{:?}",point);
   let x = point.x * point.y;
   println!("{:?}", x);
}
