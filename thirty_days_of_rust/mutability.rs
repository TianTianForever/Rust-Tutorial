use std::cell::Cell;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: Cell<i32>,
}

#[derive(Debug)]
pub struct PointRef<'a> {
    pub x: &'a mut i32,
    pub y: &'a mut i32,
}

#[derive(Debug)]
pub struct Origin<T> {
    pub x: T,
    pub y: T,
}

pub struct Node;
pub struct Edge;
pub struct MyGraph;

pub trait Graph {
    type N;  // Node
    type E;  // Edge
    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edge(&self, &Self::N) -> Vec<Self::E>;
}

impl Graph for MyGraph {
    type N = Node;
    type E = Edge;
    fn has_edge(&self, node1: &Node, node2: &Node) -> bool {
        true
    }

    fn edge(&self, node: &Node) -> Vec<Edge> {
        Vec::new()
    }
}
