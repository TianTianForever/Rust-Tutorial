use std::ops::Deref;
use std::cell::Cell;

struct Rectangle { height: f64, width: f64 }

impl Deref for Rectangle {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.height
    }
}
 
struct GenRectangle<T> {height: Cell<T>, width: T }
// Deref coercion.
impl <T> Deref for GenRectangle<T> {
    type Target = Cell<T>;
    fn deref(&self) -> &Cell<T> {
        &self.height
    }
}

fn main() {
    let rectangle = Rectangle {
        height: 2.2,
        width: 2.2,
    };
    assert_eq!(2.2, *rectangle);
  
    let genrectangle = GenRectangle {
        height: Cell::new(2.4),
        width: 2.4,
    };
    assert_eq!(Cell::new(2.4), *genrectangle);
}
