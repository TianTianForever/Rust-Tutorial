#[derive(Clone, Copy)]
pub struct Book {
    // '&'static str' is a reference to a string allocated 
    //in read only memory.
    pub author: &'static str,
    pub title: &'static str,
    pub year: u32,
}

pub fn borrow_book(book: &Book) {
    println!("I can't edit title '{}' and year '{}'.", book.title, book.year);
}

pub fn new_edition(book: &mut Book) {
    book.year = 2017;
    println!("I can edit title '{}' and year '{}'.", book.title, book.year);
}

// Calculate string length.
pub fn calculate_length(s: &String) -> usize {
    s.len()
}

// Change string length.
pub fn change(s: &mut String) {
    s.push_str("Change string length");
}

// Define struct type 'Point'.
pub struct Point{
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn take_x(&self) -> &i32 {
        &self.x
    }

    pub fn take_y(&self) -> &i32 {
        &self.y
    }
}

