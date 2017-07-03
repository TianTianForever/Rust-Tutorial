use std::cell::Cell;
use std::ops::Deref;
use std::option::Option;
use std::fmt::{Display, Debug};
use std::collections::HashMap;
#[derive(Debug)]
pub struct Person {
    pub age: Cell<i32>,
    pub name: Option<String>,
    pub height: Box<f64>,
    pub weight: Box<f64>,
    pub sex: String,
}
impl Deref for Person {
    type Target = Cell<i32>;
    fn deref(&self) -> &Cell<i32> {
        &self.age
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Account<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
pub struct AccountInfo<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

pub type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;
pub fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}, Password: {}", username, password);
    println!("Attempting logon..");
    let logon = Account {username: username, password: password };
    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        },
        _ => println!("Login Failed"), 
    }
}

/*
pub struct MyCell<T> {
    value: UnsafeCell<T>,
}
impl<T: Copy> MyCell<T> {
    // Returns a copy of the contained value.
    pub fn get(&self) -> T {
        unsafe{ *self.value.get() }
    }
}
impl<T: Copy> Clone for MyCell<T> {
    fn clone(&self) -> MyCell<T> {
        MyCell::new(self.get())
    }
}
*/
