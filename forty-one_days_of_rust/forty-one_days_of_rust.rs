use std::ops::Mul;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

struct Tree {
    leaf: i32,
    branch: RefCell<Vec<Rc<Tree>>>,
}
impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tree {{leaf: {}, branch: {:?}}}", self.leaf, self.branch)
    }
}

#[derive(Debug)] struct Point<T> { x: T, y: T }

impl<T: Mul<Output=T>> Mul for Point<T> {
    type Output = Point<T>;
    fn mul(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

fn main() {
    let leaf = Rc::new( Node {
        value: 5,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(
        Node {
            value: 5,
            children: RefCell::new(vec![leaf.clone()]),
        }
    );
    println!("{:?}", Rc::strong_count(&leaf));    
    println!("{:?}", branch);
    let tree = Rc::new( Tree {
        leaf: 10000,
        branch: RefCell::new(vec![]),
    });
    let tree_branch = Rc::new( Tree {
        leaf: 9999,
        branch: RefCell::new(vec![tree.clone()]),
    });
    println!("{:?}", tree_branch);
}
