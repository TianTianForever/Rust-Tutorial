fn main(){
    //Tuples type
    let x = (1,"hello");
    let y:(i32, &str) = (1, "hello");
    println!("x is {:?}, y is {:?}",x,y);

    // Destructuring let
    let (x1,y1,z1) = (1,2,3);
    println!("x1 is {}, x2 is {}, x3 is {}",x1, y1 , z1);

    let a = (0,);      // single-element tuple
    let b = 0;         // let b = (0);
    println!("{}",b);  // print "0"

    // Tuple indexing
    let tuples = (1,2,3,4);
    let a1 = tuples.0;
    let a2 = tuples.1;
    let a3 = tuples.2;
    let a4 = tuples.3;
    println!("a1 is {0} ,a2 is {1} ,a3 is {2} ,a4 is {3}",a1,a2,a3,a4);
    println!("{}",add_one(1));

    // fuctions
}
fn add_one(x: i32) -> i32{
    x+1
}
