use std::fmt;
use std::fmt::Debug;

pub struct User<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub active: bool,
    pub sign_in_count: u64,
}
pub fn build_user<'a>(email: &'a str, username: &'a str) -> User<'a> {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Manually implementing 'Debug'.
impl <'a> fmt::Debug for User<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "User {{email: {}, username: {}, active: {}, sign_in_count: {} }}",
            self.email, self.username, self.active, self.sign_in_count,
        )
    }
}

// Maunally implementing 'Display'.
impl <'a> fmt::Display for User<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "User ({}, {}, {}, {})",
            self.email, self.username, self.active, self.sign_in_count
        ) 
    }
}

#[derive(Debug)]
pub struct RefPoint<'a, T: 'a> {
    pub x: &'a T,
    pub y: &'a T,
}

pub fn my_print<'a, T>(t: &'a T) where 
    T: Debug + 'a {
    println!("'my_print' t: {:?}", t);
}
