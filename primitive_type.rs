fn main() {
    //Bool type
    let x = true;
    let y = false;

    //Char type
    let a = 'x';
    let b = 'y';

    //Numeric type
    let c = 33 ;
    let d: i32 = 33;
    //There are many numeric types i8, i16, i32, i64, isize,
    // u8, u16, u32, u64, usize, f32, f64
    let e = 32.0;

    //Array type
    let ar = [1,2,3];    // ar: [i32; 3]
    let mut m = [1,2,3]; // m: [i32; 3]
    println!("ar has {} elements",ar.len());
    let names = ["linda","bob","jack"];
    println!("The second name is {}",names[1]);

    //Slices
    let sli = [1,2,3,4,5,6];
    let numbers = &sli[..]; // A slices containing all of the elements in a
    println!("{:?}",numbers);
    let num = &sli[1..4];   // A slices of a: just the elements 2,3, and 4
    println!("{:?}",num);
}
