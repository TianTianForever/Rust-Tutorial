#![crate_type="lib"]
#![crate_name="associated_type"]

// Concrete type.
pub struct Yesterday;
pub struct Today;
pub struct Tomorrow;
pub struct Future;
pub struct MyFuture;

pub trait Me {
     type Yesterday;
     type Today;
     type Tomorrow;
     type Future;
     fn from_yesterday_to_today(&self,&Self::Yesterday, &Self::Today) -> Box<String>;     
}

impl Me for MyFuture {
    type Yesterday = Yesterday;
    type Today = Today;
    type Tomorrow = Tomorrow;
    type Future = Future;
    fn from_yesterday_to_today(&self, yesterday: &Yesterday, today: &Today) -> Box<String> {
        Box::new(String::from("hard work."))
    }
}
pub fn now<M: Me>(me: &M, yesterday: &M::Yesterday, today: &M::Today) -> Box<String> {
    Box::new(String::from("Ability"))
}
pub trait Time {
    type T;
    type F;
    fn create_timeline(&self, &Self::T, &Self::F) -> Option<String>;
}
impl Time for MyFuture {
    type T = Tomorrow;
    type F = Future;
    fn create_timeline(&self, tomorrow: &Tomorrow, future: &Future) -> Option<String> {
        Some(String::from("I'm going to create the timeline."))
    }
}
