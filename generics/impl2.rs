#[derive(Debug)]
struct Person {
    height: i32,
    weight: i32,
}

struct Point<T> {
    x: T,
    y: T,
}

// Implement of Person
impl Person {
    fn height(&self) -> &i32 {
        &self.height
    }

    fn weight(&self) -> &i32 {
        &self.weight
    }
}

// Implement of Point for a generic type 'T'.
impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let tom = Person {height: 172, weight: 67};
    println!("Tom: height: {:?}, weight: {:?}", tom.height(), tom.weight());
    let point = Point {x: 1.0f64, y: 2.0f64};
    println!("x:{}, y: {}", point.x(), point.y());
}
