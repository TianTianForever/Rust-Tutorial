use std::result::Result;
use std::fmt;
/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
impl <T, E> Result<T, E> {
    fn is_ok(&self) -> bool {
        match *self {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl <T, E: fmt::Debug> Result<T, E> {
    fn unwrap(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => unwrap_failed("called 'Result::unwrap()' on an 'Err' value", e),
        }       
    }
 
    // Panics if the value is an 'Err', with a panic message including the passed
    //message, and the content of the 'Err'.  
    fn expect(self, msg: &str) -> T {
        math self {
            Ok(t) => t,
            Err(e) => unwrap_failed(msg, e), 
        }
    }
}
*/

fn main() {

    // type aliases
    type MyString = String;
    let s: MyString = String::from("This is an alias of String type");

    // pub enum Result<T, E> { 
    //     Ok(T),
    //     Err(E),
    // }
    type MyResult = Result<i32, String>;
    let x: MyResult = Ok(22);
    assert_eq!(x.is_ok(), true);
   
    let z: Result<i32, &str> = Ok(1234567);
    assert_eq!(z.unwrap(), 1234567);

    // Called 'Result::unwrap()' on an 'Err' value.
    // Panics if the value is an 'Err', with a panic message provided by the 'Err's value.
    let p: Result<f64, &str> = Ok(1.001);
    p.unwrap();   // assert_eq(p.unwrap(), 1.001);
/*
    let y: Result<i32, &str> = Err("Emergency failure");
    y.unwrap();   // Panics with 'emergency failure'.
*/    
    let e: Result<u32, &str> = Err("Emergency failure");
    e.expect("Testing expect");
}
