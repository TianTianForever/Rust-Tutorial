use std::ops::Deref;
use std::fmt::Debug;
use std::option::Option;
use std::cell::Cell;

struct Point;
struct Edge;
struct MyGraph;

trait Graph {
    type P;
    type E;
    fn has_edge(&self, &Self::P, &Self::P) -> String;
    fn has_node(&self, &Self::P) -> Vec<Self::E>;
}
impl Graph for MyGraph {
    type P = Point;
    type E = Edge;
    fn has_edge(&self, point1: &Point, point2: &Point) -> String {
        "From point1 to point2.".to_string()
    }
    fn has_node(&self, p: &Point) -> Vec<Edge> {
        Vec::new()
    }

}

#[derive(Debug)]
struct Origin {
    x: Option<i32>,
    y: Option<i32>,
    z: Cell<i32>,
}
impl Deref for Origin {
    type Target = Option<i32>;
    fn deref(&self) -> &Option<i32> {
      &self.x;
      &self.y
    }
}
/*
fn never_return() -> ! { }
*/
fn main() {
    let my_graph = MyGraph;
    let object = Box::new(my_graph) as Box<Graph<P=Point, E=Edge>>;
    let origin = Origin { x: Some(1992), y: Some(1993), z: Cell::new(1994) };
    println!("{:#?}", origin);
    assert_eq!(Some(1993), *origin);
    origin.z.set(99999);
    println!("{:#?}", origin);
}
