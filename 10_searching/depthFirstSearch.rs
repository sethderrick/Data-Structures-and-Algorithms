use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Option<Rc<RefCell<Node>>>;

/// Represents a single node within a binary search tree.
struct Node {
    value: i32,
    left: TreeLink,
    right: TreeLink,
}

impl Node {
    /// Constructs a new node with the specified value.
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}

/// Represents a binary search tree.
struct BinarySearchTree {
    root: TreeLink,
}

impl BinarySearchTree {
    /// Initializes a new empty Binary Search Tree.
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// Inserts a value into the binary search tree.
    fn insert(&mut self, value: i32) {
        let new_node = Node::new(value);
        match self.root {
            Some(ref mut root) => {
                let mut current = root.clone();
                loop {
                    let mut current_borrowed = current.borrow_mut();
                    if value < current_borrowed.value {
                        if let Some(ref left) = current_borrowed.left {
                            current = left.clone();
                        } else {
                            current_borrowed.left = Some(new_node.clone());
                            break;
                        }
                    } else {
                        if let Some(ref right) = current_borrowed.right {
                            current = right.clone();
                        } else {
                            current_borrowed.right = Some(new_node.clone());
                            break;
                        }
                    }
                }
            }
            None => {
                self.root = Some(new_node);
            }
        }
    }

    /// Recursively traverses the tree in in-order.
    fn in_order(&self) -> Vec<i32> {
        fn in_order_helper(node: &TreeLink, acc: &mut Vec<i32>) {
            if let Some(ref n) = node {
                let n_borrowed = n.borrow();
                in_order_helper(&n_borrowed.left, acc);
                acc.push(n_borrowed.value);
                in_order_helper(&n_borrowed.right, acc);
            }
        }

        let mut result = Vec::new();
        in_order_helper(&self.root, &mut result);
        result
    }

    /// Recursively traverses the tree in pre-order.
    fn pre_order(&self) -> Vec<i32> {
        fn pre_order_helper(node: &TreeLink, acc: &mut Vec<i32>) {
            if let Some(ref n) = node {
                let n_borrowed = n.borrow();
                acc.push(n_borrowed.value);
                pre_order_helper(&n_borrowed.left, acc);
                pre_order_helper(&n_borrowed.right, acc);
            }
        }

        let mut result = Vec::new();
        pre_order_helper(&self.root, &mut result);
        result
    }

    /// Recursively traverses the tree in post-order.
    fn post_order(&self) -> Vec<i32> {
        fn post_order_helper(node: &TreeLink, acc: &mut Vec<i32>) {
            if let Some(ref n) = node {
                let n_borrowed = n.borrow();
                post_order_helper(&n_borrowed.left, acc);
                post_order_helper(&n_borrowed.right, acc);
                acc.push(n_borrowed.value);
            }
        }

        let mut result = Vec::new();
        post_order_helper(&self.root, &mut result);
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

    println!("In-Order: {:?}", tree.in_order());
    println!("Pre-Order: {:?}", tree.pre_order());
    println!("Post-Order: {:?}", tree.post_order());
}
