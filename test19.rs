// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f64);

// A struct with two fields.
struct Point {
    x: i32,
    y: i32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // Instantiate a 'Point'
    let point: Point = Point {x: 1, y: 2};

    // Access the fields of the Point.
    println!("point coordinates: ({}, {})",point.x, point.y);

   // Destructure the point using a 'let' binding.
   let Point {x: my_x, y: my_y} = point;
   let rectangle = Rectangle {
       // struct instantiation is an expression too
       p1: Point { x: my_y, y: my_x},
       p2: point,
   };
   
   // Instantiate a tuple a unit struct.
   let nil = Nil;

   // Instantiate a tuple struct.
   let pair = Pair(1, 2.01);

   // Access the fields of a tuple struct.
   println!("pair contains {:?}, and {:?}.", pair.0, pair.1);

   // Destructure a tuple struct.
   let Pair(integer, decimal) = pair;
   println!("pair contains {:?} and {:?}",integer, decimal);   
}
