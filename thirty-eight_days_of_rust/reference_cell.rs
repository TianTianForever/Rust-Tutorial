use std::cell::RefCell;

fn immutably_borrows(a: &i32) {
    println!("a is {:?}", a);
}

fn mutably_borrows(b: &mut i32) {
    *b += 1;
}

fn reference_cell(r: &RefCell<i32>) {
    immutably_borrows(&r.borrow());
    mutably_borrows(&mut r.borrow_mut());
    immutably_borrows(&r.borrow()); 
}

fn value(value: &i32) {
    println!("value: {:?}", value);
}

fn add_value(value: &mut i32) {
    *value += 1;
}

fn look_at_result(result: &RefCell<i32>) {
    value(&result.borrow());
    add_value(&mut result.borrow_mut());
    value(&result.borrow());
}

enum KeyBoard {
    W,
    A,
    S,
    D,
}

// Implement Default.
impl Default for KeyBoard {
    fn default() -> KeyBoard {
        KeyBoard::A;
        KeyBoard::D
    }
}
fn main() {
    let data = RefCell::new(11111111);
    reference_cell(&data);
    let value = RefCell::new(99);
    look_at_result(&value);
    let  a = RefCell::new(1993);
    println!("{:?}", a);
    *a.borrow_mut() = 1994;
    println!("{}", *a.borrow());   
}
