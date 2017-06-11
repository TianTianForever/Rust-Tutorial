use std::ops::{Add, Sub, Mul, Div};

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

// Implement Addition.
impl Add for Point { 
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

// Implement subtraction.
impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point {
             x: self.x - rhs.x,
             y: self.y - rhs.y,
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
fn divide(x: i32, y: i32) -> Option<i32> {
    if (y == 0) | (x == 0) {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
   let point = Point::new(1, 2); 
   println!("{:?}",point);
   let x = point.x * point.y;
   println!("{:?}", x);
   let point_1 = Point {x: 30, y: 20};
   let point_2 = Point {x: 10, y: 10};
   let point_3 = Point {x: point_1.x - point_2.x, y: point_1.y - point_2.y};
   println!("{:?} - {:?} = {:?}",point_1, point_2, point_3);
}
