mod myadd;
use myadd::*;
mod mysub;
use mysub::*;
mod mymul;
use mymul::*;
mod mydiv;
use mydiv::*;

use std::num::ParseIntError;

type Result<T> = std::result::Result<T, CalculateError>;
enum CalculateError {
     DivisionByError,
     Parse(ParseIntError),
}

fn main() {

    // Add
    let point1 = Point { x: 1.0, y: 1.0 };
    let point2 = Point { x: 2.1, y: 2.1 };
    assert_eq!(Point {x: 1, y: 1} + Point{x: 2, y: 2},
               Point{x: 3, y: 3});
    let point3 = point1 + point2;
    println!("{:#?} + {:#?} = {:?}", point1, point2, point3);
    println!(" ");
    let origin1 = Origin{ x: 1, y: 1 };
    let origin2 = Origin{ x: 2, y: 2 };
    let origin3 = origin1 + origin2;
    println!("{:#?} + {:#?} = {:?}", origin1, origin2, origin3);
    println!(" ");
    // Sub
    let subpoint1 = SubPoint { x: 2.1, y: 2.1 };
    let subpoint2 = SubPoint { x: 0.01, y: 0.02 };
    let subpoint3 = subpoint1 - subpoint2;
    println!("{:#?} - {:#?} = {:?}", subpoint1, subpoint2, subpoint3);
    let genvalue1 = GenericValue {x: 1, y: 1};
    println!(" ");
    // Mul
    let mulpoint1 = MulPoint { x: 1.1, y: 1.1 };
    let mulpoint2 = MulPoint { x: 1.1, y: 1.1 };
    let mulpoint3 = mulpoint1 * mulpoint2;
    println!("{:#?} * {:#?} = {:?}", mulpoint1, mulpoint2, mulpoint3);
    println!(" ");
    // Div
    let divvalue1 = DivValue { x: 2.222, y: 2.222 };
    let divvalue2 = DivValue { x: 1.0, y: 1.0 };
    let divvalue3 = divvalue1 / divvalue2;
    println!("{:#?} / {:#?} = {:?}", divvalue1, divvalue2, divvalue3);
    assert_eq!(Area{x: 2, y: 2} * Area{x: 2, y: 2}, Area{x: 4, y: 4});
}
