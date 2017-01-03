//struct type
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let mut point = Point {x: 0, y: 0};
    
    // tuple struct
    struct Color(u8, u8, u8);
    let color = Color(0xa4, 0xc6, 0x39);
    let Color(red, gree, blue) = color;

    // A tuple sturct's constructors can be used as function.
    struct Digit(i32);
    let v = vec![0,1,2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();

    //newtype: a tuple struct with only one element
    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;

    //unit-like stucts
    struct EmptyStruct;
    let empty = EmptyStruct;
}
