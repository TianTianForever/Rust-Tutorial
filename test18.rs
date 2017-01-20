use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}
#[derive(Debug)]
struct Matrix(f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.0, self.1, self.2)
    }
}

fn main() {
    //A tuple with a bunch of different types
    let long_tuple = (1u8, 11u16, 111u32, 1111u64,
                      -1i8, -11i16, -111i32, -111i64,
                      0.1f32, 0.11f64, 'a', false);
    //Values can be extracted from the tuple using tuple indexing.
        println!("{:?}",long_tuple.0);
        println!("{:?}",long_tuple.1);
    //    ...
    
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed is {:?}", reverse(pair));
  
    //Both 'Debug' and "Display" were implemented .
    let matrix = Matrix(1.1, 1.2, 1.3);
    println!("{:?}",matrix);
    println!("{}", matrix);    
 
    // Tuples can be tuple members
    let tuple_of_tuples = ((11u8, 111u16, 1111u32), (22u32, 222u64), -222i32);
    println!("{:?}", tuple_of_tuples);
}
