extern crate errors;
use errors::*;

fn add(x: i32) -> Option<i32> {
    Some(x + x)
}

fn square(x: u32) -> Option<u32> {
    Some(x * x)
}

fn nothing(_: f64) -> Option<f64> {
    None
}
/*
pub trait ToOwned {
    type Owned: Borrow<Self>;

    // Creates owned data from borrowed data.
    fn to_owned(&self) -> Self::Owned;
}

// Maps a Result<T, E> to Result<T, E> by applying a function to a contained 
// Err value, leaving an Ok value untouched. 
fn map_err<F, O>(self, op: O) -> Result<T, F> where
    O: FnOnce(E) -> F,
*/

fn main() {
    is_work();
    println!("{:?}", Some(2).and_then(square).and_then(square));
    assert_eq!(Some(2).and_then(add).and_then(add), Some(8));
    assert_eq!(Some(2.0).and_then(nothing).and_then(nothing), None);
    assert_eq!(None.and_then(square).and_then(square), None);
   
    // Using method 'to_owned()'.
    let string: &str = "TianTianForever";
    let string2: String = string.to_owned();
    println!("{}", string2);

    // Using method 'map_err()'.
    fn wrong(x: i32) -> String { format!("error code: {}", x)}
    let value: Result<i32, i32> = Ok(1993);
    println!("{:?}", value.map_err(wrong));
    let value2: Result<i32, i32> = Err(1992);
    println!("{:?}", value2.map_err(wrong));
    // Create two files
    setup();
    // Concat contents of two files
    match concat("a.txt", "b.txt") {
        Ok(o) => println!("Concat contents of 'a.txt' and 'b.txt': '{}", o),
        Err(e) => println!("{}", e),
    }    
}
