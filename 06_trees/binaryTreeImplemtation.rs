/// A node in the binary search tree, containing a value and pointers to the left 
/// and right child nodes.
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

/// A Binary Search Tree (BST) with operations for inserting, looking up, and removing elements.
struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: PartialOrd + Copy> BinarySearchTree<T> {
    /// Constructs a new, empty BinarySearchTree.
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// Inserts a new value into the binary search tree.
    /// 
    /// # Arguments
    ///
    /// * `value` - The value to be inserted into the tree.
    pub fn insert(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None,
        });

        if self.root.is_none() {
            self.root = Some(new_node);
        } else {
            let mut current = self.root.as_mut().unwrap();
            loop {
                if value < current.value {
                    if current.left.is_none() {
                        current.left = Some(new_node);
                        break;
                    } else {
                        current = current.left.as_mut().unwrap();
                    }
                } else {
                    if current.right.is_none() {
                        current.right = Some(new_node);
                        break;
                    } else {
                        current = current.right.as_mut().unwrap();
                    }
                }
            }
        }
    }

    /// Looks up a value in the binary search tree and returns a reference to the node 
    /// containing it if found.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to look for in the tree.
    pub fn lookup(&self, value: T) -> Option<&Node<T>> {
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

    /// Removes a value from the binary search tree.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to remove from the tree.
    pub fn remove(&mut self, value: T) -> bool {
        let mut current = self.root.as_mut();
        let mut parent = None;

        while let Some(ref mut found) = current {
            if value < found.value {
                parent = current;
                current = found.left.as_mut();
            } else if value > found.value {
                parent = current;
                current = found.right.as_mut();
            } else {
                // Element found: handle the removal.
                if found.right.is_none() {
                    let new_link = found.left.take();
                    if let Some(parent_node) = parent {
                        if parent_node.value > value {
                            parent_node.left = new_link;
                        } else {
                            parent_node.right = new_link;
                        }
                    } else {
                        self.root = new_link;
                    }
                } else if found.right.as_ref().unwrap().left.is_none() {
                    let right_child = found.right.take().unwrap();
                    right_child.left = found.left.take();
                    if let Some(parent_node) = parent {
                        if parent_node.value > value {
                            parent_node.left = Some(right_child);
                        } else {
                            parent_node.right = Some(right_child);
                        }
                    } else {
                        self.root = Some(right_child);
                    }
                } else {
                    // Find the smallest value in the right subtree.
                    let (smallest, smallest_parent) = Self::find_min(&mut found.right.as_mut().unwrap());
                    smallest.left = found.left.take();
                    smallest.right = found.right.take();
                    if let Some(parent_node) = smallest_parent {
                        parent_node.left = smallest.right.take();
                    }
                    if let Some(parent_node) = parent {
                        if parent_node.value > value {
                            parent_node.left = Some(smallest);
                        } else {
                            parent_node.right = Some(smallest);
                        }
                    } else {
                        self.root = Some(smallest);
                    }
                }
                return true;
            }
        }
        false
    }

    /// Helper function to find the minimum node and its parent in a subtree.
    fn find_min(node: &mut Box<Node<T>>) -> (Box<Node<T>>, Option<&mut Box<Node<T>>>) {
        let mut current = node;
        let mut parent = None;

        while let Some(ref mut next) = current.left
