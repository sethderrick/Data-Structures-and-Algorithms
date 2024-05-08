use std::collections::VecDeque;

/// Represents a single node in a binary search tree.
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    /// Constructs a new node with the specified value.
    ///
    /// # Arguments
    /// * `value` - The integer value of the node.
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

/// Represents a binary search tree.
struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    /// Initializes a new empty Binary Search Tree.
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// Inserts a new value into the binary search tree.
    ///
    /// # Arguments
    /// * `value` - The value to insert into the tree.
    ///
    /// This method places the new value in the appropriate position in the tree
    /// to maintain the BST property where the left subtree has only values less than
    /// the node value, and the right subtree only values greater.
    fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node::new(value));
        if let Some(ref mut root) = self.root {
            let mut current = root;
            loop {
                if value < current.value {
                    if let Some(ref mut left) = current.left {
                        current = left;
                    } else {
                        current.left = Some(new_node);
                        break;
                    }
                } else {
                    if let Some(ref mut right) = current.right {
                        current = right;
                    } else {
                        current.right = Some(new_node);
                        break;
                    }
                }
            }
        } else {
            self.root = Some(new_node);
        }
    }

    /// Searches for a value in the tree and returns a reference to the node containing that value if it exists.
    ///
    /// # Arguments
    /// * `value` - The value to search for in the tree.
    ///
    /// # Returns
    /// An optional reference to the node containing the value, or None if the value is not found.
    fn lookup(&self, value: i32) -> Option<&Node> {
        let mut current = self.root.as_ref();
        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Less => current = node.left.as_ref(),
                Ordering::Greater => current = node.right.as_ref(),
                Ordering::Equal => return Some(node),
            }
        }
        None
    }

    /// Removes a value from the tree, if it exists, and maintains the BST properties.
    ///
    /// # Arguments
    /// * `value` - The value to remove from the tree.
    ///
    /// # Returns
    /// True if the node was successfully removed, or false if the node was not found.
    fn remove(&mut self, value: i32) -> bool {
        self.root = self._remove(self.root.take(), value)
    }

    /// A helper function to facilitate removal of a node with the specified value.
    ///
    /// This function recursively finds and removes the node, and then restructures the tree if necessary.
    ///
    /// # Arguments
    /// * `node` - The current node being inspected (as an option containing a boxed node).
    /// * `value` - The value to remove.
    ///
    /// # Returns
    /// An optional boxed node that replaces the current node in its parent's child reference.
    fn _remove(&self, node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        node.map(|mut node| {
            match value.cmp(&node.value) {
                Ordering::Less => node.left = self._remove(node.left, value),
                Ordering::Greater => node.right = self._remove(node.right, value),
                Ordering::Equal => {
                    return match (node.left, node.right) {
                        (Some(left), None) => Some(left),
                        (None, Some(right)) => Some(right),
                        (None, None) => None,
                        (Some(left), Some(mut right)) => {
                            if right.left.is_none() {
                                right.left = Some(left);
                                right
                            } else {
                                let mut leftmost = &mut right;
                                while let Some(ref mut next_left) = leftmost.left {
                                    leftmost = next_left;
                                }
                                std::mem::swap(&mut leftmost.left, &mut node.left);
                                leftmost.right = Some(right);
                                Some(left)
                            }
                        }
                    };
                }
            }
            Some(node)
        })
    }

    /// Performs a breadth-first search (BFS) on the tree and prints each node's value.
    ///
    /// This method uses a queue to traverse the tree level by level, ensuring each node is visited in the order of depth.
    fn breadth_first_search(&self) {
        let mut queue = VecDeque::new();
        if let Some(ref root) = self.root {
            queue.push_back(root);
            while let Some(node) = queue.pop_front() {
                println!("{}", node.value);
                if let Some(ref left) = node.left {
                    queue.push_back(left);
                }
                if let Some(ref right) = node.right {
                    queue.push_back(right);
                }
            }
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
    if tree.remove(170) {
        println!("Node 170 removed");
    }
    println!("Breadth First Search Results:");
    tree.breadth_first_search();
}
