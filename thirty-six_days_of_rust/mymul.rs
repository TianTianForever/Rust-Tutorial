use std::ops::Mul;
/*
pub trait Mul<RHS = Self> {
    typpe Output;
    fn mul(self, rhs: RHS) -> Self::Output;
}
*/

#[derive(Debug, Clone, Copy)]
pub struct MulPoint<T> {
    pub x: T,
    pub y: T,
}

impl <T: Mul<Output=T>> Mul for MulPoint<T> {
    type Output = MulPoint<T>;
    fn mul(self, rhs: MulPoint<T>) -> MulPoint<T> {
        MulPoint {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Area {
    pub x: i32,
    pub y: i32,
}
impl Mul for Area {
    type Output = Area;
    fn mul(self, rhs: Area) -> Area {
        Area {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl PartialEq for Area {
    fn eq(&self, rhs: &Area) -> bool {
        (self.x == rhs.x) && (self.y == rhs.y)
    }
}
