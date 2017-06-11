use std::ops::{Sub, Add};

/*
trait Add<RHS=Self> {
    // The resulting type after applying the '*' operator.
    type Output;
    
    // The method for the '+' operator.
    fn Add(self, rhs: RHs) -> Self::output;
}
*/

#[derive(Debug)]
struct Origin {
    x: usize,
    y: usize,
}

impl Origin {
    fn new(x: usize, y: usize) -> Self {
        Origin {
            x: x,
            y: y,
        }
    }
}

// Implement Addition.
impl Add for Origin {

    type Output = Self;
    fn add(self, rhs: Self) -> Self {
         Origin::new(self.x + rhs.x, self.y + rhs.y)
    }
}

// Implement Subtraction.
impl Sub for Origin {
    type Output = Origin;
    fn sub(self, rhs: Origin) -> Origin {
        Origin {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
fn main() {
    let origin = Origin{x: 100, y: 150};
    let origina = Origin{x: 50, y: 100};
    let origin_sub = Origin{
        x: origin.x - origina.x,
        y: origin.y - origina.y,
    };
    println!("{:?} - {:?} = {:?}", origin, origina, origin_sub);
}
