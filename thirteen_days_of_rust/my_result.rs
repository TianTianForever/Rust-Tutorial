/*
// Result is a type that represents either success(Ok) or failure(Err).
enum Result<T, E> {
    Ok(T),  // Contains the success value.
    Err(E), // Contains the error value.
}

impl <T, E> Result<T, E> {
    // Returns true if the result is Ok.
    fn is_ok(&self) -> bool {
        match *self {
            Ok(_)  => true,
            Err(_) => false
        }
    }
    
    // Returns false if the result is Error.
    fn is_err(&self) -> bool {
        !self.is_ok()
    }

    // Converts from 'Result<T, E>' to 'Option<T>'.
    fn ok(&self) -> Option<T> {
        match self {
            Ok(x)  => Some(x),
            Err(_) => None,
        }
     }
    
     // Converts from 'Result<T, E>' to 'Option<E>'.
     fn err(&self) -> Option<T> {
         match self() -> Option<E> {
             Ok(_)  => None,
             Err(x) => Some(x),
         }
     }
}
*/

fn main() {   
    // Using 'is_ok' method.
    let x: Result<i32, &str> = Ok(11111);
    assert_eq!(x.is_ok(), true);
    let y: Result<i32, &str> = Err("Take some error message!!!");
    assert_eq!(y.is_ok(), false);
   
    // Using 'is_err' method.
    let a: Result<f64, &str> = Ok(12121221.1212);
    assert_eq!(a.is_err(), false);
    let b: Result<f64, &str> = Err("Take some error message!!!");
    assert_eq!(b.is_err(), true);

    // Using 'ok' method.
    let c: Result<u32, &str> = Ok(12);
    assert_eq!(c.ok(), Some(12));
    let d: Result<u32, &str> = Err("Take some error message!!!");
    assert_eq!(d.ok(), None);
    
    // Using 'err' method.
    let e: Result<f64, &str> = Ok(12222232324.1111142342352);
    assert_eq!(e.err(), None);
    let f: Result<f64, &str> = Err("Nothing here");
    assert_eq!(f.err(), Some("Nothing here"));
}
