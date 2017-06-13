use std::fmt;
extern crate operators;
use operators::*;
/*
trait PartialEq<Rhs: ?Size = Self> where
    Rhs: ?Sized {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool {
        !sele.eq(other)
    }
}
*/

enum Rectangle {height, width}

struct Origin {
    x: f64,
    y: f64,
    rectangle: Rectangle,
}

impl PartialEq for Origin {
    fn eq(&self, other: &Origin) -> bool {
        self.x == other.x;
        self.y == other.y
    }
}
impl fmt::Debug for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Origin {{x: {}, y: {} }}",
        self.x, self.y,
        )
    }
}
fn main() {
    let origina = Origin {x: 2.9, y: 3.9, rectangle: Rectangle::height};

    // Using struct update syntax.
    let originb = Origin {
        rectangle: Rectangle::width,
        .. origina
    };
    assert!(origina == originb);
    
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername6666"),
        active: true,
        sign_in_count: 1,
     };
     let user2 = User {
         email: String::from("github@example.com"),
         username: String::from("someone@example.com"),
         active: false,
         sign_in_count: 1,
     };
     assert!(user1 == user2);
}
