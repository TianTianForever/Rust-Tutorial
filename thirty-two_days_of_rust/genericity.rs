use std::option::Option;
use std::result::Result;
use std::fmt::Debug;

#[derive(Debug)]
pub enum MatchFailure {
    DivisionByZero,
    NonPositiveLogarithm,
    NegativeSquareRoot,
}

type MatchResult = Result<f64, MatchFailure>;
pub fn div (x: f64, y: f64) -> MatchResult {
    if y == 0.0 {
        Err(MatchFailure::DivisionByZero)
    } else {
        Ok(x / y)
    }
}

pub fn square_root(x: f64) -> MatchResult {
    if x < 0.0 {
        Err(MatchFailure::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

pub fn ln(x: f64) -> MatchResult {
    if x <= 0.0 {
        Err(MatchFailure::NonPositiveLogarithm)
    } else {
        Ok(x.ln())
    }
}
pub fn operate(x: f64, y: f64) -> f64 {
    match div(x, y) {
        Err(why)  => panic!("{:?}", why),
        Ok(retio) => match ln(retio) {
            Err(why)  => panic!("{:?}", why),
            Ok(ln) => match square_root(ln) {
                Err(why)  => panic!("{:?}", why),
                Ok(square_root) => square_root,
            },
        },
    }
}
pub fn take_value(x: Option<i32>) -> Option<i32> {
    let b = match x {
        Some(s) => s,
        None => 0,
    };

    Some(b)
}
