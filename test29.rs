fn main() {
    'outer: loop {
        println!("outer");
       'inner: loop {
           println!("inner");
 
           // This breaks the outer loop.
           break 'outer;
       }
       println!("this point will never be reached");
    }
    println!("exited the outer loop");
} 
