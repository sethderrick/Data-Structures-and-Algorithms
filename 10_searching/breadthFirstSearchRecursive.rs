use std::collections::VecDeque;

/// Represents a single node within a binary search tree.
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    /// Constructs a new node with the specified value.
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

    /// Inserts a value into the binary search tree.
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

    /// Looks up a value in the tree and returns a reference to the node containing that value if found.
    fn lookup(&self, value: i32) -> Option<&Node> {
        let mut current = self.root.as_ref();
        while let Some(node) = current {
            if value < node.value {
                current = node.left.as_ref();
            } else if value > node.value {
                current = node.right.as_ref();
            } else {
                return Some(node);
            }
        }
        None
    }

    /// Removes a value from the tree.
    fn remove(&mut self, value: i32) -> bool {
        let mut current = &mut self.root;
        let mut parent = None;

        while let Some(ref mut node) = current {
            if value < node.value {
                parent = Some(current);
                current = &mut node.left;
            } else if value > node.value {
                parent = Some(current);
                current = &mut node.right;
            } else {
                if node.right.is_none() {
                    *current = node.left.take();
                } else if let Some(ref mut right) = node.right {
                    if right.left.is_none() {
                        right.left = node.left.take();
                        *current = Some(right.take());
                    } else {
                        let (leftmost, leftmost_parent) = Self::detach_leftmost(right);
                        leftmost.left = node.left.take();
                        leftmost.right = node.right.take();
                        *current = Some(leftmost);
                        if let Some(leftmost_parent) = leftmost_parent {
                            leftmost_parent.left = leftmost.right.take();
                        }
                    }
                }
                return true;
            }
        }
        false
    }

    /// Helper method to detach the leftmost node from the given subtree.
    fn detach_leftmost(node: &mut Box<Node>) -> (Box<Node>, Option<&mut Box<Node>>) {
        let mut parent = None;
        let mut current = node;
        while let Some(ref mut left) = current.left {
            parent = Some(current);
            current = left;
        }
        (current.left.take().unwrap(), parent)
    }

    /// Performs an iterative breadth-first search on the tree and returns a vector of values.
    fn breadth_first_search(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(ref root) = self.root {
            queue.push_back(root);
            while let Some(node) = queue.pop_front() {
                result.push(node.value);
                if let Some(ref left) = node.left {
                    queue.push_back(left);
                }
                if let Some(ref right) = node.right {
                    queue.push_back(right);
                }
            }
        }
        result
    }

    /// Performs a recursive breadth-first search on the tree, starting with the root.
    fn breadth_first_search_r(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(ref root) = self.root {
            queue.push_back(root);
            self.bfs_recursive_helper(&mut queue, &mut result);
        }
        result
    }

    /// Helper method for recursive BFS that processes the queue until it is empty.
    fn bfs_recursive_helper(&self, queue: &mut VecDeque<&Box<Node>>, result: &mut Vec<i32>) {
        if let Some(node) = queue.pop_front() {
            result.push(node.value);
            if let Some(ref left) = node.left {
                queue.push_back(left);
            }
            if let Some(ref right) = node.right {
                queue.push_back(right);
            }
            self.bfs_recursive_helper(queue, result);
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

    println!("BFS Iterative: {:?}", tree.breadth_first_search());
    println!("BFS Recursive: {:?}", tree.breadth_first_search_r());
}
