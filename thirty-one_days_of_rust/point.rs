use std::ops::Deref;


pub struct Mp3 {
    pub audio: Vec<u8>,
    pub artist: Option<String>,
    pub title: Option<String>,
}

/*
pub trait Deref {
    // The resulting type after dereferencing
    type Target: ?Sized;
    
    // The method called to dereference a value
    fn deref(&self) -> &Self::Target;
}

impl<'a, T> Deref for &'a T {
    type Target = T;
    fn deref(&self) -> &T {
        *self
    }
}
*/

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

pub struct Origin<T> {
    pub x: T,
}

impl <T> Deref for Origin<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.x
    }
}

pub fn makefile() -> String {
    String::from("Everyone is dissatisfied with his own fortune.")
}


