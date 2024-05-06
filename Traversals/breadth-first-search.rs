/*
  Implementation of a Binary Search Tree (BST) with methods for insertion,
  lookup, breadth-first search (BFS), and recursive BFS
*/

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    val: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: PartialOrd + Debug> Node<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            left: None,
            right: None,
        }))
    }
}

#[derive(Debug)]
struct BinarySearchTree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: PartialOrd + Debug> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, val: T) {
        let new_node = Node::new(val);
        match self.root {
            None => self.root = Some(new_node),
            Some(ref root) => {
                let mut temp = root.clone();
                loop {
                    let mut temp_ref = temp.borrow_mut();
                    if new_node.borrow().val < temp_ref.val {
                        if temp_ref.left.is_none() {
                            temp_ref.left = Some(new_node.clone());
                            break;
                        } else {
                            temp = temp_ref.left.as_ref().unwrap().clone();
                        }
                    } else {
                        if temp_ref.right.is_none() {
                            temp_ref.right = Some(new_node.clone());
                            break;
                        } else {
                            temp = temp_ref.right.as_ref().unwrap().clone();
                        }
                    }
                }
            }
        }
    }

    fn lookup(&self, val: T) -> bool {
        let mut temp = self.root.clone();
        while let Some(node) = temp {
            let node_ref = node.borrow();
            if val == node_ref.val {
                return true;
            } else if val < node_ref.val {
                temp = node_ref.left.clone();
            } else {
                temp = node_ref.right.clone();
            }
        }
        false
    }
}

impl<T: PartialOrd + Debug + Copy> BinarySearchTree<T> {
    fn breadth_first_search(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            let mut queue = Vec::new();
            queue.push(root.clone());

            while !queue.is_empty() {
                let current = queue.remove(0);
                let current_ref = current.borrow();
                result.push(current_ref.val);
                if let Some(ref left) = current_ref.left {
                    queue.push(left.clone());
                }
                if let Some(ref right) = current_ref.right {
                    queue.push(right.clone());
                }
            }
        }
        result
    }

    fn recursive_bfs(&self, queue: Vec<Rc<RefCell<Node<T>>>>, mut mylist: Vec<T>) -> Vec<T> {
        if queue.is_empty() {
            return mylist;
        }
        let mut new_queue = Vec::new();
        for node in queue {
            let node_ref = node.borrow();
            mylist.push(node_ref.val);
            if let Some(ref left) = node_ref.left {
                new_queue.push(left.clone());
            }
            if let Some(ref right) = node_ref.right {
                new_queue.push(right.clone());
            }
        }
        self.recursive_bfs(new_queue, mylist)
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

    let x = tree.lookup(170);
    println!("Lookup 170: {}", x);

    let bfs = tree.breadth_first_search();
    println!("Breadth-First Search: {:?}", bfs);

    let recursive_bfs = tree.recursive_bfs(vec![tree.root.unwrap()], Vec::new());
    println!("Recursive BFS: {:?}", recursive_bfs);
}

// TODO: add tests
