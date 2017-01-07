use std::f64::consts::PI;
use std::fmt::Debug;
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
        PI * self.radius * self.radius
    }
}

// Tait constraint
fn print_area<T: HasArea>(shap: T){
    println!("This shap has an area of {}", shap.area());
}

// multiple trait bounds
fn foo<T: Clone, K: Clone + Debug>(x: T,y: K) {
    x.clone();
    y.clone();
    println!("{:?}",y)
}

fn bar<T, K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug
{
   x.clone();
   y.clone();
   println!("{:?}",y);
}

fn main() {
    let c = Circle {x: 0.0, y: 0.0, radius: 1.0};
    print_area(c);
    foo(1,2);
    bar(1,2);
}
