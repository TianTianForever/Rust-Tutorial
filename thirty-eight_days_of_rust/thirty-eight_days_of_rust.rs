use std::option::Option;

#[derive(Debug)]
struct Origin {
    x: f64,
    y: f64,
}

// Return 'Option' type.
fn take_option(x: Origin) -> Option<f64> {
    Some(x.x)
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Return 'Option' type.
fn wrap_option<T>(point: Point<T>) -> Option<T> {
    point.y;
    Some(point.x)
}

#[derive(Debug, Clone, Copy)]
struct GenValue<T, U> {
    x: T,
    y: U,
}
// Return 'Result' type.
fn wrap_result<T, U>(value: GenValue<T, U>) -> Result<T, U> {
    Ok(value.x)
}

fn main() {
    let origin = Origin {
        x: 1.0,
        y: 2.0,
    };
    assert_eq!(Some(1.0), take_option(origin));
    let point = Point { 
        x: 1.111101, 
        y: 2.222202 
    };
    assert_eq!(Some(1.111101), wrap_option(point));

    let value = GenValue {
        x: 2.1,
        y: 323232.2221,
    };
    {
        println!("{:?}", wrap_result(value));
    }
    assert_eq!(Ok(value.x), wrap_result(value));
}
