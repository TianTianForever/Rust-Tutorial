#![crate_type="lib"]
#![crate_name="wrapper"]
use std::ops::Add;
use std::fmt;

// Concrete type.
#[derive(Debug)] pub struct Future;
#[derive(Debug)] pub struct Tomorrow;
pub struct Possible;
pub struct Myself;
#[derive(Debug)] pub struct Today{ pub time: u64 }
#[derive(Debug)] pub struct Yesterday{ pub time:u64 }
pub struct Wrapper(pub Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
pub trait MyFuture {
    type T;
    type F;
    fn my_future(&self, &Self::T, &Self::F) -> Option<Possible>;
}
impl MyFuture for Myself {
    type T = Tomorrow;
    type F = Future;
    fn my_future(&self, tomorrow: &Tomorrow, future: &Future) -> Option<Possible> {
        Some(Possible)
    }
}
pub fn hard<M: MyFuture>(my_future: &M, tomorrow: &M::T, future: &M::F) -> Option<Possible> {
    Some(Possible)
}

impl Add for Today {
    type Output = Today;
    fn add(self, rhs: Today) -> Today {
        Today{ time: self.time + rhs.time }
    }
}
impl Add<Yesterday> for Today {
    type Output = Today;
    fn add(self, rhs: Yesterday) -> Today {
        Today{ time: self.time + rhs.time }
    }
}