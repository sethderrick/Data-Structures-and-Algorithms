/*
  Rust's Memory Management: Using Rc<RefCell<Node<T>>> enables shared 
  ownership and mutable references to nodes. This is crucial for recursive 
  structures like trees where nodes may have multiple owners (parent and child).

  Generics and Traits: The BinarySearchTree is implemented as a generic 
  type T which must support the PartialOrd trait for ordering comparisons 
  and Debug trait for printing.

  Option Handling: Rust's Option type is used to safely manage the existence 
  of child nodes, avoiding null pointer issues common in other languages.

  Lookup Function: Efficiently traverses the tree based on BST properties (left 
    for values less than the current node, right for greater).

  In-Order Traversal: A helper function demonstrates how to recursively traverse 
  the tree to perform actions in sorted order, crucial for understanding tree 
  structure and debugging.
*/

use std::cell::RefCell;
use std::rc::Rc;

/// A Node in a binary search tree containing a value, and optional references to left and right children.
struct Node<T> {
    data: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

/// A Binary Search Tree (BST) which organizes elements in a hierarchical structure
/// allowing efficient insertions, deletions, and lookups.
pub struct BinarySearchTree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: PartialOrd + std::fmt::Debug> BinarySearchTree<T> {
    /// Creates a new, empty `BinarySearchTree`.
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// Inserts a new element into the BST. New elements are added in a way to preserve the BST property.
    pub fn insert(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            left: None,
            right: None,
        }));
        match self.root {
            None => self.root = Some(new_node),
            Some(ref root) => {
                let mut current = root.clone();
                loop {
                    let mut curr_borrowed = current.borrow_mut();
                    if new_node.borrow().data < curr_borrowed.data {
                        if let Some(ref left) = curr_borrowed.left {
                            current = left.clone();
                        } else {
                            curr_borrowed.left = Some(new_node.clone());
                            break;
                        }
                    } else {
                        if let Some(ref right) = curr_borrowed.right {
                            current = right.clone();
                        } else {
                            curr_borrowed.right = Some(new_node.clone());
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Looks up a value in the BST. Returns true if the value exists, otherwise false.
    pub fn lookup(&self, data: T) -> bool {
        let mut current = self.root.clone();
        while let Some(node) = current {
            let borrowed_node = node.borrow();
            if data == borrowed_node.data {
                return true;
            } else if data < borrowed_node.data {
                current = borrowed_node.left.clone();
            } else {
                current = borrowed_node.right.clone();
            }
        }
        false
    }

    /// Helper function to perform in-order traversal of the BST.
    fn in_order_traversal(&self, node: Option<Rc<RefCell<Node<T>>>>, f: &mut dyn FnMut(&T)) {
        if let Some(ref n) = node {
            let borrowed_n = n.borrow();
            self.in_order_traversal(borrowed_n.left.clone(), f);
            f(&borrowed_n.data);
            self.in_order_traversal(borrowed_n.right.clone(), f);
        }
    }

    /// Prints all elements of the BST in sorted order.
    pub fn print_tree(&self) {
        self.in_order_traversal(self.root.clone(), &mut |data| {
            println!("{:?}", data);
        });
    }
}

fn main() {
    let mut bst = BinarySearchTree::new();
    bst.insert(10);
    bst.insert(5);
    bst.insert(6);
    bst.insert(12);
    bst.insert(8);
    let x = bst.lookup(6);
    println!("Found 6?: {}", x);
    let y = bst.lookup(99);
    println!("Found 99?: {}", y);
    println!("BST in sorted order:");
    bst.print_tree();
}

// TODO: add tests