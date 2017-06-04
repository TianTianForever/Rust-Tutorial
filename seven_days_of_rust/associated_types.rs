use std::fmt::Display;

// Define struct types 'Node' 'Edge' and 'MyGraph'.
pub struct Node;
pub struct Edge;
pub struct MyGraph;

// Define 'Graph' trait.
pub trait Graph {
    type N;
    type E;
    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

// Implement associated types.
impl Graph for MyGraph {
    type N = Node;
    type E = Edge;
    
    fn has_edge(&self, node1: &Node, node2: &Node) -> bool {
        true
    }

    fn edges(&self, node: &Node) -> Vec<Edge> {
        Vec::new()
    }
}
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub struct Point(pub i32, pub i32);
pub trait MyPoint{
    type X;
    type Y;
    fn contains(&self, &Self::X, &Self::Y) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl MyPoint for Point {
    type X = i32;
    type Y = i32;
    fn contains(&self, x: &i32, y: &i32) -> bool {
        (&self.0 == x) && (&self.1 == y)
    }
    fn first(&self) -> i32 {
        self.0
    } 
    fn last(&self) -> i32 {
        self.1
    }
}
pub fn difference<c: MyPoint>(my_point: &c) ->i32 {
     my_point.last() - my_point.first()
}

