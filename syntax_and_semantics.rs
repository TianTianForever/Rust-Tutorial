fn main() {
    //Variable bindings
    let x = 5;
    let y = 5;
    println!("x is {0},y is {1}",x,y);
    //Patterns
    let (a,b) = (1,2);
    println!("a is {0},b is {1}",a,b);
    //type annotations
    let c: i32 = 5;
    println!("{:?}",c);
    //immutable
    let d = 33;
    //mutable
    let mut e = 10; //mut x: i32
    e = 11 ;
    println!("{:?}", e);
    //initializing bngdings
    //Error,rust warns us that we never use the variable binding.
    //let cf: i32;
    //println!("{:?}",cf );
}
