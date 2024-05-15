use std::cmp::Ordering;

/// A Node struct that stores a value and links to left and right children in a binary search tree.
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

/// A BinarySearchTree struct that uses a linked list of Nodes to store elements in a binary search tree.
#[derive(Debug)]
struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    /// Constructs a new, empty BinarySearchTree.
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// Inserts a new value into the binary search tree.
    pub fn insert(&mut self, value: T) {
        let mut current = &mut self.root;
        while let Some(ref mut found) = current {
            match value.cmp(&found.value) {
                Ordering::Less => current = &mut found.left,
                Ordering::Greater => current = &mut found.right,
                Ordering::Equal => return, // Ignore duplicates
            }
        }
        *current = Some(Box::new(Node::new(value)));
    }

    /// Finds a value in the binary search tree and returns a reference to it, if it exists.
    pub fn lookup(&self, value: T) -> Option<&Node<T>> {
        let mut current = &self.root;
        while let Some(ref found) = current {
            match value.cmp(&found.value) {
                Ordering::Less => current = &found.left,
                Ordering::Greater => current = &found.right,
                Ordering::Equal => return Some(found),
            }
        }
        None
    }

    /// Deletes a value from the binary search tree, if it exists.
    pub fn remove(&mut self, value: T) {
        self.root = BinarySearchTree::remove_node(self.root.take(), value);
    }

    fn remove_node(node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {
        let mut node = node?;
        match value.cmp(&node.value) {
            Ordering::Less => {
                node.left = BinarySearchTree::remove_node(node.left.take(), value);
                Some(node)
            }
            Ordering::Greater => {
                node.right = BinarySearchTree::remove_node(node.right.take(), value);
                Some(node)
            }
            Ordering::Equal => match (node.left.take(), node.right.take()) {
                (None, None) => None,
                (Some(left), None) => Some(left),
                (None, Some(right)) => Some(right),
                (Some(left), Some(right)) => {
                    let (min_right, right) = BinarySearchTree::extract_min(right);
                    node.value = min_right.value;
                    node.right = right;
                    node.left = Some(left);
                    Some(node)
                }
            },
        }
    }

    fn extract_min(mut node: Box<Node<T>>) -> (Box<Node<T>>, Option<Box<Node<T>>>) {
        if let Some(left) = node.left.take() {
            let (min_left, left) = BinarySearchTree::extract_min(left);
            node.left = left;
            (min_left, Some(node))
        } else {
            let right = node.right.take();
            (node, right)
        }
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
    tree.remove(170);

    if let Some(node) = tree.lookup(20) {
        println!("Found node with value: {}", node.value);
    } else {
        println!("Node not found.");
    }
}
