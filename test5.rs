fn main() {
    let day = 5;
    match day { 
        1 | 6 => println!("weekend"),
        w @ 1 ... 5 => println!("weekday {}",w),
        _ => println!("invalid"),
    }
   
    let a = 5;
    match a {
        // the 'r' inside the match has the type '&i32'.
        ref r => println!("{}",r),
    }
    
    let mut b = 6;
    match b {
        //The 'mr' inside the match has the type 'i32' and is mutable.
        ref mr => println!("{}",mr),
    }

   let c = (0, 1);
   match c {
       (0, y) => println!("x is '0' and y is {:?}", y),
       (x, 1) => println!("x is '{:?}' and y is '1'", x),
       _ => println!("It doesn't matter what they are"),
   }
   
   struct Point {
       x: i32,
       y: i32,
   }
   let origin = Point {x: 0, y: 0};
   match origin {
       Point {x, ..} => println!("x is {:?}", x),
   }

   enum OptionalInt {
       Value(i32),
       Message,
   }
   let x = OptionalInt::Value(5);
   match x {
       OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five"),
       OptionalInt::Value(..) => println!("Got an int"),
       OptionalInt::Message => println!("No such luck"),
   }
   
   let number = Some(7);
   let mut optional = Some(0);
   // If 'let' destructures 'number' into 'Some(i)', evaluate the block.
   if let Some(i) = number {
       println!("Matched {:?}", i);
   } else {
       println!("Didn't match a number");
   }

   //While 'let' destructure 'number' into 'Some(i)', evaluate the block.
   while let Some(i) = optional {
       if i > 9 {
           println!("Geater than 9, quit!");
           optional = None;
       } else {
           println!("'i' is {:?}. try again.",i);
           Some(i + 1);
       }
   }
}
