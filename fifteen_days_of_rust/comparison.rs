extern crate iterator;
use iterator::*;
use std::option::Option;
use std::cmp::{PartialEq};
/*
trait PartialEq<Rhs: ?Sized = Self> {
    // This method tests for 'self' and 'other' values to be equal, 
    // and is used by '=='.
    fn eq(&self, other: &Rhs) -> bool;
  
    // This method tests for '!='.
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}
*/
enum BookFormat {
    Paperback,
    Hardback,
    Ebook
}
#[allow(dead_code)]
struct Book {
    isbn: i32,    // international standard book number
    format: BookFormat,
}
impl PartialEq for Book {
    fn eq(&self, other: &Book) -> bool {
        self.isbn == other.isbn
    }
}

fn main() {
    fn add(x: i32, y: i32) -> Option<i32> {
        if x + y == 0 {
            None
        } else {
            Some(x + y)
        }
    }
    println!("{:?}",add(1, 2));
    let book1 = Book {
        isbn: 10,
        format: BookFormat::Paperback,
    };
    
    let book2 = Book {isbn: 22, format: BookFormat::Hardback};
    let book3 = Book {
        isbn: 22,
        format: BookFormat::Ebook,
    };
    assert!(book2 == book3);
    assert!(book1 != book2);
    let v: Vec<i32> = vec![1, 2, 3];
    println!("{:?}",my_map(v));
}
