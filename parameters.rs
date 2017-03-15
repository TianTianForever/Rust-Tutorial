use std::mem;
fn square<T>(x: T) -> f64 where
    T: Fn(f64) -> f64 {
    x(2.2)
}
fn beautiful_gril<T>(x: T)
    where T: FnOnce() {
    x();
}
fn main() {
    let double = |x| x * 3.0;
    println!("square: {}", square(double));
    let name = "Anny";
    let mut weight = "100".to_owned();

    let info = || {
        // The closure captures by reference(&T). Requires 'Fn'.
        println!("This name is {}.", name);
        weight.push_str("!!!");
        // The closure captures by mutable reference(&mut T). Requires 'FnMut'.
        println!("Weight: {}", weight);
        // The closure captures by value(T). requires 'FnOnce'.
        mem::drop(weight);
    };
    beautiful_gril(info);
} 
