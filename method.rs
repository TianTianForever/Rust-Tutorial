fn divisible(a: u32, b: u32) -> bool { 
    if a == 0 {
        return false;
    }
    a % b == 0
}

fn number(n: u32) -> () {
    if divisible(n, 15) {
        println!("{:?} is divisible by 15", n);
    } else if divisible(n, 5) {
        println!("{:?} is divisible by 5", n);
    } else if divisible(n, 3) {
        println!("{:?} is divisible by 3", n);
    } else {
        println!("{:?}", n);
    }
}

fn repeatedly(n: u32){
    for n in 1..n + 1 {
        number(n);
    }
}

struct Point {
    x: f64,
    y: f64,
}
// Implementation block, all 'point' methods go in here.
impl Point {
    // This is a static method.
    // Static methods don't need to be called by an instance
    //These methods are generally used as constructors.
    fn origin() -> Point {
      Point { x: 0.0, y: 0.0 }
    }
    
   // Another static method, taking two arguments:
   fn new(x: f64, y: f64) -> Point {
       Point { x: x, y: y }
   }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is an instance method
    // '&self' is sugar for 'self: &Self', where 'Self' is the type of the
    // caller object. in this case 'Self' = 'Rectangle'.
    fn area(&self) -> f64 {
        // 'self' gives access to the struct fields via the dot operator.
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        // 'abs' is a 'f64' method that returns the absolute value of the caller
        ((x1 - x2) * (y1 - y2)).abs()
    }
    fn perimeter(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
    // This method requires the caller object to be mutable
    // '&mut self' desugars to 'self: &mut Self'
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
       
        self.p1.y += y;
        self.p2.y += y;
    }
}

// 'Pair' owns resources: two heap allocated integers.
struct Pair(Box<i32>, Box<i32>);
impl Pair {
    // This method 'consumes' the resources of the caller object
    // 'self' desugars to 'self: Self'
    fn destroy(self) {
        // Destructure 'self'
        let Pair(first, second) = self;
        println!("Destroying pair({}, {})",first, second);
        // 'first' and 'second' go out of scope and get freed.
    }
}

fn main() {
    repeatedly(100);
 
    let rectangle = Rectangle {
        // Static methods are called using double colons;
        p1: Point::origin(),
        p2: Point::new(1.0, 2.0),
    };
    // Instance methods are called using the dot operator
    // Note that the first argument '&self' is implicitly passed, i.e.
    // 'rectangle.perimeter()' == 'rectangle.perimeter(&rectangle)'
    println!("rectangle area is: {}", rectangle.area());
    println!("rectangle perimeter: {}", rectangle.perimeter());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(1.0, 1.0);
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
} 
