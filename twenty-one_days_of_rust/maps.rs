/*
// Applies a function to the contained value(if any),
// or returnss a 'default'(if no).
fn map(U, F: FnOnce(T) -> U)(self, f: F) -> Option<U> {
    match self {
        Some(x) => Some(f(x)),
        None => None,
    }
}
*/

#[derive(Debug)] enum Value {X, Y, Z}
#[derive(Debug)] struct PX(Value);
#[derive(Debug)] struct PY(Value);
#[derive(Debug)] struct PZ(Value);
fn take_x(v: Option<Value>) -> Option<PX> {
    match v {
        Some(v) => Some(PX(v)),
        None => None,
    }
}

fn _x(v: Option<Value>) -> Option<PX> {
    v.map(|value| PX(value))
}

fn take_y(px: Option<PX>) -> Option<PY> {
    match px {
        Some(PX(v)) => Some(PY(v)),
        None => None,
    }
}

fn _y(px: Option<PX>) -> Option<PY> {
    px.map(|PX(value)| PY(value))
}

fn take_z(py: Option<PY>) -> Option<PZ> {
    match py {
        Some(PY(v)) => Some(PZ(v)),
        None => None,
    }
}

fn _z(py: Option<PY>) -> Option<PZ> {
    py.map(|PY(value)| PZ(value))
}
fn final_z(v: Option<Value>) -> Option<PZ> {
    v.map(|value| PX(value))
     .map(|PX(value)| PY(value))
     .map(|PY(value)| PZ(value))
}

fn main() {
    println!("PROCESS");
    let maybe_some_string = Some(String::from("twenty-one"));
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(10));

    let x = Some(Value::X);
    let y = Some(Value::Y);
    let z = Some(Value::Z);
    final_z(z);
    let xxxx = take_x(x);
    let yyyy = take_y(xxxx);
    let zzzz = take_z(yyyy);
}
