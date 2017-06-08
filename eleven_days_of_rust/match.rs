// A match block can destructure items.

fn main() {
    let point = (1.0, 2.0);
    println!("take some information about point: {:?}.", point);  

    // Match can be use to destructure tuple 'point'.
    #[allow(illegal_floating_point_literal_pattern)]
    match point {
        (1.0, y) => println!("first is '1.0' and last is {:?}", y),
        (x, 0.0) => println!("first is {:?} and last is '0'", x),
           _   => println!("It doesn't matter they are"),
    }

   // match can be use to destructure enum 'Color'.
   #[allow(dead_code)]
   enum Color {
       Red,
       Green,
       Blue,
       RGB(u32, u32, u32),
   }
   let color = Color::RGB(255, 255, 255);
   match color {
       Color::Red => println!("This color is Red!"),
       Color::RGB(r, g, b) =>
           println!("Red: {}, Green: {}, Blue: {}.",r, g, b),
       _ => println!("Ignore something."),
   }

   // struct can be destructured as shown.
   struct Origin {
       x: i32,
       y: i32,
   }
   
   let origin = Origin {
       x: 1,
       y: 2,
   };
   let Origin{x, y} = origin;
   println!("Take some information about Origin: x = {}, y = {}",x, y);
}
