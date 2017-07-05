use std::ops::Sub;
/*
pub trait Sub<RHS = Self> {
    type Output;
    fn sub(self, rhs: RHS) -> Self::Output;
}
*/

#[derive(Debug, Clone, Copy)]
pub struct SubPoint<T> {
    pub x: T,
    pub y: T,
}

impl <T: Sub<Output=T>> Sub for SubPoint<T> {
    type Output = SubPoint<T>;
    fn sub(self, rhs: SubPoint<T>) -> SubPoint<T> {
        SubPoint {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    } 
}

pub struct GenericValue<T> {
    pub x: T,
    pub y: T,
}

impl <T: Sub<Output=T>> Sub for GenericValue<T> {
    type Output = GenericValue<T>;
    fn sub(self, rhs: GenericValue<T>) -> GenericValue<T> {
        GenericValue {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}


