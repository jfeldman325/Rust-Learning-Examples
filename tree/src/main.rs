use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    fn new(value: i32, children: RefCell<Vec<Rc<Node>>>, parent: RefCell<Weak<Node>>) -> Node {
        Node {
            value,
            children,
            parent,
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Node with value {} went out of scope", self.value)
    }
}
fn main() {
    let leaf = Rc::new(Node::new(
        3,
        RefCell::new(vec![]),
        RefCell::new(Weak::new()),
    ));
    let root = Rc::new(Node::new(
        6,
        RefCell::new(vec![Rc::clone(&leaf)]),
        RefCell::new(Weak::new()),
    ));

    *leaf.parent.borrow_mut() = Rc::downgrade(&root);
    println!(
        "{:?}",
        match leaf.parent.borrow().upgrade() {
            Some(x) => x.value,
            None => -1,
        }
    );
}
