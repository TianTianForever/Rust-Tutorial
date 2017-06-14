use std::ops::{Div, Sub};

use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Value {
    x: usize,
    y: usize
}
impl Value {
    fn new(x: usize, y: usize) -> Value {
        Value {
            x: x,
            y: y,
        }
    }
} 
impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
       let x = self.x / rhs.x;
       let y = self.x / rhs.y;
       Value::new(x, y)
    }
}
impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Value::new(x, y)
    }
}
fn main() {
    let v: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x + 1).collect();
    println!("{:?}", v);
    let value1 = Value {x: 1, y: 2};
    let value2 = Value {x: 1, y: 2};
    let value3 = Value {
        x: value1.x / value2.x,
        y: value2.y / value2.y  
    };

    {
        println!("{:?} / {:?} = {:?}", value1, value2, value3);
    }
    let value4 = Value {
        x: value1.x - value2.x,
        y: value2.y - value2.y,
    };
    println!("{:?} - {:?} = {:?}", value1, value2, value4);
}
