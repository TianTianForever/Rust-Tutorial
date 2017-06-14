use std::ops::Mul;

struct Origin {value: usize}

#[derive(Debug)]
struct Vector {value: Vec<usize>}

impl Mul<Vector> for Origin {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {value: rhs.value.iter().map(|v| self.value * v).collect()}
    }
}
impl PartialEq<Vector> for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[test]
fn test_vector() {
    let origin = Origin{value: 3};
    let vector = Vector{value: vec![1, 3, 5]};
    assert_eq!(origin * vector, Vector{value: vec![3, 9, 15]});
}

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point {
            x: x,
            y: y,
        }
    } 
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let x = self.x * rhs.x;
        let y = self.y * rhs.x;
        Point::new(x, y)
    }
}

#[test]
fn it_works() {
    let point1 = Point {x: 2, y: 2};
    let point2 = Point {x: 2, y: 2};
    let point3 = Point {
        x: point1.x * point2.x,
        y: point1.y * point2.y,
    };
    let point4 = Point::new(1, 2); 
    println!("{:?}",point3);
}
