// Associated types

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub trait Point {
    type X;
    type Y;
    fn point(&self, &Self::X, &Self::Y) -> bool;
    fn position_x(&self) -> i32;
    fn position_y(&self) -> i32;
}

impl Point for Position {
    type X = i32;
    type Y = i32;
    fn point(&self, x: &i32, y: &i32) -> bool {
        (&self.x == x) && (&self.y == y)
    }
    fn position_x(&self) -> i32 { self.x }
    fn position_y(&self) -> i32 { self.y }
}

pub fn using_associated_types<Z: Point>(z: &Z) -> i32 {
    z.position_y() - z.position_x()
}

pub struct Value(pub f64, pub f64);

pub trait CollectionValue {
    type one;
    type two;
    fn take_value_one(&self) -> f64;
    fn take_value_two(&self) -> f64;
    fn colletion_value(&self, &Self::one, &Self::two) -> bool;
}

impl CollectionValue for Value {
    type one = f64;
    type two = f64;
    fn take_value_one(&self) -> f64 {
        self.0
    }
    fn take_value_two(&self) -> f64 {
        self.1
    }
    fn colletion_value(&self,one: &f64, two: &f64) -> bool {
        (&self.0 == one) && (&self.1 == two)
    }
}
pub fn collection<three: CollectionValue>(t: &three) -> f64 {
    t.take_value_two() - t.take_value_one()
}
