//    let i = 1;
//    fn increment(i: i32) -> i32 { i + 1}     
//    closure_explicitly_anotated = |i: i32| -> { i + 1 };
//    closure_inferred = |i| i + 1 ;   

// When taking a closure as an input parameter, the closure's
// complete type must be annotated using one of a few trais.

// Fn: the closure captures by reference   &T,
// FnMut: the closure captures by mutable reference   &mut T,
// FnOnce: the closure captures by value  T,

pub fn my_print<F> (f: F) where
    F: FnOnce() {
    f();
}

pub fn print_string<S> (s: S)
    where S: FnOnce() -> String {
    println!("{}", s());
    
}
// let mut x = 5;
// {
//     let mut square_x = || x *= x;
//     square_x();
// }
// assert_eq!(x, 25);

pub fn do_twice<T>(mut t: T) where
    T: FnMut() {
    t();
    t();
}

// 'Fn' trait takes an immutable reference to any captured variable.
// let square = |x| x * x;
// assert_eq!(square(5), 25);

pub fn square<F>(f: F) -> i32 
    where F: Fn(i32) -> i32 {
    f(5)
}

// let double = |x| x * 2;
// assert_eq!(call_with_one(double), 2);
pub fn call_with_one<F>(f: F) -> usize 
    where F: Fn(usize) -> usize {
    f(1)
}

// let one = |x| x + 1;
// assert_eq!(add_one(one), 2);
pub fn add_one<T>(t: T) -> usize where
    T: Fn(usize) -> usize {
    t(1)
}

fn call_with_ref<F>(some_value: F) -> i32 
    where F: for<'a> Fn(&'a i32) -> i32 {
    let value = 0;
    some_value(&value)
}
