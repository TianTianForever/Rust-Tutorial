use std::ops::Div;

/*
pub trait Div(RHS = Self) {
    type Output;
    fn div(self, rhs: RHS) -> Self::Output;
}
*/

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DivValue<T> {
    pub x: T,
    pub y: T,
} 

impl <T: Div<Output=T>> Div for DivValue<T> {
    type Output = DivValue<T>;
    fn div(self, rhs: DivValue<T>) -> DivValue<T> {
        DivValue {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
