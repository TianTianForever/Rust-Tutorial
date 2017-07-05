use std::ops::Add;
#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output=T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Point<T>) -> Point<T> {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Origin { pub x: i32, pub y: i32 }

impl Add for Origin {
    type Output = Origin;
    fn add(self, rhs: Origin) -> Origin {
        Origin {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub trait MyAdd<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

#[derive(Debug, Clone, Copy)]
pub struct Value { pub x: i32, pub y: i32 }

impl MyAdd for Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Value {
        Value {
            x: self.x + rhs.y,
            y: self.y + rhs.y,
        }
    }
}
