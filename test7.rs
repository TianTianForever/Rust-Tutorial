use std::f64::consts::PI;
//'impl' 'struct' 'enum' 'trait' , implement 'method call syntax' and 'associated function'.
trait HasArea {
    fn area(&self) -> f64;
}
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
impl HasArea for Circle {
    fn area(&self) -> f64 {
        PI * (self.radius * self.radius) 
    }
}
impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
   
   fn area(&self) -> f64 {
        PI * (self.radius * self.radius)
   }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}
impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {:?}",shape.area());
}
fn main() {
    let c = Circle {x: 0.0, y: 0.0, radius: 1.0};
    println!("{:?}",c.area());
   
    //use associated function and chaining
    println!("{:?}",Circle::new(0.0, 0.0, 1.0).area());

    let s = Square {x: 0.0, y: 0.0, side: 3.0};
    print_area(s);
}

