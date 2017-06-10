use std::net::TcpStream;

/*
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash )]
enum Option<T> {
    None,      // No value.
    Some(T),   // Some value T. 
}
impl<T> Option<T> {
    // Return 'ture' if the option is a 'Some' value.
    fn is_some(&self) -> bool {
        match *self {
            Some(_) => true,
            None => false, 
        }
    }

    // Return 'true' if the option is a 'None' value.
    fn is_none(&self) -> bool {
        !self.is_some()
    }

}
*/
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
      Some(x / y)
    }
}

fn main() {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
  
    // Using 'is_some()' function.
    let a: Option<i32> = Some(1212312);
    assert_eq!(a.is_some(), true);
    let b: Option<i32> = None;
    assert_eq!(b.is_some(), false);

    // Using 'is_none()' function.
    let c: Option<f64> = Some(123.321);
    assert_eq!(c.is_none(), false);
    let d: Option<f64> = None;
    assert_eq!(d.is_none(), true);
   
    // The return value of function is an option.
    let result = divide(3333.1111, 33423432.1);
    match result {
    // The division was valid.
    Some(x) => println!("Result: {}", x),
    // The division was invalid.
    None    => println!("Connot divide by 0"),
    }
}
