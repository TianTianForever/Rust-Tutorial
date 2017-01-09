//generic
/*
enum Option<T> {
    Some(T),
    None,
}
*/

//generic functions
fn make_pair<T, E>(x: T, y: E) ->(T, E) {
    (x, y)
}

// generic structs
struct Point<T>{
    x: T,
    y: T,
}

fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<f64> = Some(5.0f64);
    let couple = make_pair("male", "female");
    let integer_origin = Point {x: 0, y: 0};
    let float_origin = Point {x: 0.0, y: 0.0};    
}
