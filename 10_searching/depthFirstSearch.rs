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
        fn insert_helper(node: &TreeLink, value: i32) -> TreeLink {
            match node {
                None => Some(Node::new(value)),
                Some(ref n) => {
                    let mut n_borrowed = n.borrow_mut();
                    if value < n_borrowed.value {
                        n_borrowed.left = insert_helper(&n_borrowed.left, value);
                    } else {
                        n_borrowed.right = insert_helper(&n_borrowed.right, value);
                    }
                    node.clone()
                }
            }
        }

        self.root = insert_helper(&self.root, value);
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
