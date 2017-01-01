fn main() {
    //boolean type
    let t = true;
    let f: bool = false;

    // char type
    let c = 'c';
  
    //numeric types
    let x = 32;
    let y: u32 = 123_456_789_0;
    let z: f64 = 1.23e+2;
    let zero = z.abs_sub(123.4);
    let bin = 0b1111_0000;
    let oct = 0o1234_1234;
    let hex = 0xf23a_b049;
    
    //string types
    let str = "I don't care what is wrriten in your history as long as you are here with you";
    let mut String = str.to_string();

    //arrary and slices 
    let a = [0,1,2,3,4];
    let middle = &a[1..4];
    let mut ten_zeros: [i64; 10] = [0; 10];
    
    //tuples
    let tuple: (i32, &str) = (50, "hello");
    println!("{:?}",tuple);
    let (fifty, _) = tuple;
    let hello = tuple.1;
    println!("{:?}",hello);

    //raw pointers
    let x = 5;
    let raw = &x as *const i32;
    let points_as = unsafe { *raw };
    
    // functions  
    fn foo (x: i32) -> i32 { x }
    let bar: fn(i32) -> i32 = foo;
    
    //explicit conversion
    let decimal = 12.1234_f32;
    let integer = decimal as u8;
    println!("{:?}",integer);
    let character = integer as char;
    println!("{:?}",character);
    
    // type aliases
    type NanoSecond = u64;
    type Point = (u8, u8);
    
    

}
