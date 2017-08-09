extern crate future;
use future::*;
use std::cell::Cell;
use std::num::ParseIntError;
use std::error;
// Generic types.
trait GMyWorld<Land, Sky, River, Tree, Plant, Animal> {
    // ...   
}

/*
type Result<T> = std::result::Result<T, Box<error::Error>>;

fn gen_my_world<L, S, R, T, P, A, G: GMyWorld<L, S, R, T, P, A>>() -> std::result::Result<i32> {
    Ok(1)
}
*/

fn gen_my_world<L, S, R, T, P, A, W: GMyWorld<L, S, R, T, P, A>>(my_world: &W, 
    land: &L, sky: &S, river: &R, tree: &T, plant: &P, animal: &A) -> Cell<String> {
    Cell::new(String::from("Generic implementation my world."))
}

// Associated type.
trait MyWorld {
     type Land;
     type Sky;
     type River;
     type Tree;
     type Plant;
     type Animal;
}

fn my_world<M: MyWorld>(my_world: &M, land: &M::Land, sky: &M::Sky, river: &M::River, 
                        tree: &M::Tree, plant: &M::Plant, animal: &M::Animal) -> Option<String> {
    Some(String::from("Implement my world."))
}

fn partial_my_world<M: MyWorld>(my_world: &M, land: &M::Land, sky: &M::Sky, plant: &M::Plant) -> Cell<Option<String>> {
    Cell::new(Some(String::from("Partial implementation.")))
} 

fn else_my_world(my_world: &MyWorld<Land=usize, Sky=usize, River=usize, Tree=usize, Plant=usize, Animal=(usize, usize)>) {

}

fn main() {

}
