struct Point {
    x: i32,
    y: i32,
}

struct PointRef<'a> {
    x:&'a mut i32,
    y:&'a mut i32,
}
fn main() {
    let mut origin = Point {x: 0, y: 0};



/*    {
        let a = &mut origin;
        *a = Point{x: 1, y: 1};
    }
    println!("({} {})",origin.x,origin.y);

''''
    origin.x = 1;
    origin.y = 1;
    println!("(x:{}, y:{})",origin.x,origin.y);
*/

    {
        let r = PointRef {x: &mut origin.x, y: &mut origin.y};
        *r.x = 5;
        *r.y = 5;
    }
    println!("({} {})",origin.x,origin.y);
}
