use std::option::Option;
use std::result::Result;

struct Tomorrow;   // Concrete type.
struct Futures;   // Concrete type.

// Generic type.
trait GWork<T, F> {
    // ...
}

// Associated type.
trait AWork {
    type Tomorrow;
    type Futures;   
}

fn is_hard_work<T, F, G: GWork<T, F>>(work: &G, today: &T, tomorrow: &F) -> std::option::Option<Futures> {
    // ...
    match work {
        _ => None,
    }
}

fn is_hard_study<T, F, G: GWork<T, F>>(work:&G, tomrrow: &T, future: &F) -> std::option::Option<Tomorrow> {

    match work {
        _ => None,
    }
}

fn is_ability_important<T, F, G: GWork<T, F>>(work: &G, tomorrow: &T, future: &T) -> Box<i32> {
    Box::new(1)
}

// Associated Type.
fn whatever<W: AWork>(work: &W, tomorrow: &W::Tomorrow, futures: &W::Futures) -> u32 {
   1 + 1
}

fn leave_here<W: AWork>(work: &W, tomorrow: &W::Tomorrow, futures: &W::Futures) -> std::option::Option<i32> {
    Some(1992)
}
fn main() {

}
