// Of course traits can also be generic.
struct Apple;

struct Empty;

struct Null;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<U, T> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {

    }
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
    //empty;
    //null;

    let point = Point {x: 1.22f64, y: 1.22222222f64};
    println!("{}, {}", point.x, point.y);
}
