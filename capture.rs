use std::mem;
fn main() {
    let color = "Green";
    // A closure to print 'color' which immediately borrows ('&')
    // color and stores the borrow and closure in the 'print' variable,
    // It will remain borrowed until 'print' goes out of scope.
    // 'println!' only requires ' by reference' so it doesn't 
    // impose anything more restrictive.
    let print = || println!("color: {}", color);
    
    // Call the closure using the borrow.
    print();
    print();

   let mut count = 0;
   // A closure to increment 'count' could take either '&mut count'
   // or 'count' but '&mut count' is less restrictive so it takes that.
   // Immediately borrows 'count'.

   // A 'mut' is required on 'inc' because a '&mut' is stored inside.
   // Thus. calling the closure mutates the closure which requires a 'mut'.
   let mut inc = || {
       count += 1;
       println!("count: {}",count);
   };
   inc();
   inc();
  
   // A non-copy type.
   let movable = Box::new(3);
   // 'mem::drop' requires 'T' so this must take by value, 
   // A copy type would copy into the closure  leaving the original untouched.
   // A non-copy must move and so 'moveable' immediately moves into the closure.
   let consume = || {
       println!("movable: {:?}", movable);
       mem::drop(movable);
   };
   consume();
   //consume();
   // ^ TODO: Try uncommenting this line. 
}
