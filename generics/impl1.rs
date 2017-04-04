#[derive(Debug)]
struct Val {
    val: f64
}

struct Origin<T> {
    x: T,
    y: T,
}

// Implement of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// Implement of Origin for a generic type 'T'.
impl <T>Origin<T> {
    fn point_x(&self) -> &T {
        &self.x
    }

    fn point_y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let v = Val { val: 1.09872 };
    println!("{:?}", v);

    let o = Origin {x: 1.0f64, y: 1.0f64 };
    println!("{}, {}", o.point_x(), o.point_y());
}
