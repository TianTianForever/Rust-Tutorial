// execuable.rs
// Link to 'library', import items under the 'three_days_of_rust' module.
extern crate three_days_of_rust;

fn main() {
    let point = three_days_of_rust::Point(1.0f64, 1.01f64);
    println!(
        "The point x is {}, y is {}",
        point.take_x(),
        point.take_y(),
    );

    // Create a variable 'float_64' of type 'Value<f64>'.
    let float_f64 = three_days_of_rust::Value(3.33333f64);
    
    let rectangle = three_days_of_rust::Rectangle {
        length: 10.0f64,
        width: 11.1111f64,
    };
    three_days_of_rust::print_area(rectangle);

    let circle = three_days_of_rust::Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };
    
    three_days_of_rust::print_area(circle);
}
