/*
  Rust's memory safety features require us to manage memory and ownership
  more explicitly than in many other languages, so we will use Rc<RefCell<Node>>
  to manage mutable borrowings of nodes.

  Nodes are managed by Rc<RefCell<Node<T>>> to allow shared ownership and
  mutability.

  Implements in-order, pre-order, and post-order traversals recursively,
  demonstrating Rust's capabilities to handle complex data structures and
  recursion elegantly.
*/

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Node<T> {
    val: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: PartialOrd> Node<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            left: None,
            right: None,
        }))
    }
}

struct BinarySearchTree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: PartialOrd> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, val: T) {
        let new_node = Node::new(val);
        match self.root {
            None => self.root = Some(new_node),
            Some(ref root) => {
                let mut current = root.clone();
                loop {
                    let mut node = current.borrow_mut();
                    if new_node.borrow().val < node.val {
                        if node.left.is_none() {
                            node.left = Some(new_node.clone());
                            break;
                        } else {
                            current = node.left.as_ref().unwrap().clone();
                        }
                    } else {
                        if node.right.is_none() {
                            node.right = Some(new_node.clone());
                            break;
                        } else {
                            current = node.right.as_ref().unwrap().clone();
                        }
                    }
                }
            }
        }
    }

    fn lookup(&self, val: T) -> bool {
        let mut current = self.root.clone();
        while let Some(node) = current {
            let node_borrow = node.borrow();
            if val == node_borrow.val {
                return true;
            } else if val < node_borrow.val {
                current = node_borrow.left.clone();
            } else {
                current = node_borrow.right.clone();
            }
        }
        false
    }
}

impl<T: PartialOrd + Clone> BinarySearchTree<T> {
    fn inorder(&self) -> Vec<T> {
        fn inorder_traversal<T: Clone>(node: &Option<Rc<RefCell<Node<T>>>>, acc: &mut Vec<T>) {
            if let Some(ref n) = node {
                inorder_traversal(&n.borrow().left, acc);
                acc.push(n.borrow().val.clone());
                inorder_traversal(&n.borrow().right, acc);
            }
        }
        let mut result = vec![];
        inorder_traversal(&self.root, &mut result);
        result
    }

    fn preorder(&self) -> Vec<T> {
        fn preorder_traversal<T: Clone>(node: &Option<Rc<RefCell<Node<T>>>>, acc: &mut Vec<T>) {
            if let Some(ref n) = node {
                acc.push(n.borrow().val.clone());
                preorder_traversal(&n.borrow().left, acc);
                preorder_traversal(&n.borrow().right, acc);
            }
        }
        let mut result = vec![];
        preorder_traversal(&self.root, &mut result);
        result
    }

    fn postorder(&self) -> Vec<T> {
        fn postorder_traversal<T: Clone>(node: &Option<Rc<RefCell<Node<T>>>>, acc: &mut Vec<T>) {
            if let Some(ref n) = node {
                postorder_traversal(&n.borrow().left, acc);
                postorder_traversal(&n.borrow().right, acc);
                acc.push(n.borrow().val.clone());
            }
        }
        let mut result = vec![];
        postorder_traversal(&self.root, &mut result);
        result
    }
}

fn main() {
    let mut tree = BinarySearchTree::new();
    tree.insert(9);
    tree.insert(4);
    tree.insert(6);
    tree.insert(20);
    tree.insert(170);
    tree.insert(15);
    tree.insert(1);

    println!("In-order traversal: {:?}", tree.inorder());
    println!("Pre-order traversal: {:?}", tree.preorder());
    println!("Post-order traversal: {:?}", tree.postorder());
}

// TODO: add tests
