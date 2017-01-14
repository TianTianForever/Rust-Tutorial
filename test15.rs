use std::fmt;

// Define a structure named 'List' containing a 'Vec'.
struct List(Vec<i32>);

// Implement 'Display' for 'List'.
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Dereference and create a reference to 'vec' via destructuring.
        let List(ref vec) = *self;
        try!(write!(f, "["));

        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, and a comma
            // before calling 'write!' use "try!"
            if count != 0 { try!(write!(f, ", "));}
            try!(write!(f, "{}", v));
        }
        write!(f, "]")
    }
}
fn main() {
    let v = List(vec![1, 2, 3, 4]);
    println!("{}",v);
}
