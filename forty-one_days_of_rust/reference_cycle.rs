use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::fmt;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ Node value: {}, parent: {:?}, children: {:?} }}", self.value, self.parent, self.children)
    }
}

#[derive(Debug)]
struct Tree {
    value: i32,
    branch: RefCell<Weak<Tree>>,
    leaf: RefCell<Vec<Rc<Tree>>>,
}

fn main() {
    let leaf = Rc::new( Node {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // Try to get a reference to the parent of leaf by using the 'upgrade' method.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new( Node {
        value: 11,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![leaf.clone()]),
    });
    // Using the Rc::downgrade function to create a 'Weak' reference to 'branch' from the 'Rc' in 'branch'.
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let tree_leaf = Rc::new( Tree {
        value: 10,
        branch: RefCell::new(Weak::new()),
        leaf: RefCell::new(vec![]),
    });
    println!("tree_leaf strong = {:?}, weak = {:?}", Rc::strong_count(&tree_leaf), Rc::weak_count(&tree_leaf));
    let tree_branch = Rc::new( Tree {
        value: 11,
        branch: RefCell::new(Weak::new()),
        leaf: RefCell::new(vec![tree_leaf.clone()]),
    });
    *tree_leaf.branch.borrow_mut() = Rc::downgrade(&tree_branch);        
    println!("tree_branch strong = {:?}, weak = {:?}", Rc::strong_count(&tree_branch), Rc::weak_count(&tree_branch));
}
