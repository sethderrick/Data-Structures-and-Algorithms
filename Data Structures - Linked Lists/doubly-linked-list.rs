/*
  Key Points in the Rust Implementation:

  Rc and RefCell: Used for reference-counted and mutable access to nodes.

  Weak: Used for the prev pointers to avoid creating reference cycles.

  Safety and Correctness: Rust’s strict type system and ownership model 
  ensure that the list manages memory safely and avoids issues like dangling
  pointers.

  Option Handling: Rust's Option type is used extensively to handle the 
  presence and absence of nodes, ensuring that accesses are checked for 
  null values, which helps prevent runtime errors.
*/


use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A node in a doubly linked list, which holds a value and pointers to the next and previous nodes.
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

/// A doubly linked list with methods to manipulate its nodes.
pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    length: usize,
}

impl<T> DoublyLinkedList<T> {
    /// Creates a new, empty `DoublyLinkedList`.
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// Appends a node with the specified value to the end of the list.
    pub fn append(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: self.tail.clone().map(Rc::downgrade),
        }));
        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }
        self.tail = Some(new_node);
        self.length += 1;
    }

    /// Prepends a node with the specified value to the beginning of the list.
    pub fn prepend(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.clone(),
            prev: None,
        }));
        if let Some(head) = self.head.take() {
            head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        }
        self.head = Some(new_node.clone());
        if self.length == 0 {
            self.tail = Some(new_node);
        }
        self.length += 1;
    }

    /// Inserts a node with the specified value at a given index.
    pub fn insert(&mut self, index: usize, data: T) {
        if index == 0 {
            self.prepend(data);
            return;
        }
        if index >= self.length {
            self.append(data);
            return;
        }

        let leader = self.traverse_to_index(index - 1).unwrap();
        let follower = leader.borrow().next.clone().unwrap();
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: Some(follower.clone()),
            prev: Some(Rc::downgrade(&leader)),
        }));
        follower.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        leader.borrow_mut().next = Some(new_node);
        self.length += 1;
    }

    /// Removes a node at a given index.
    pub fn remove(&mut self, index: usize) {
        if index == 0 {
            if let Some(new_head) = self.head.as_ref().unwrap().borrow().next.clone() {
                new_head.borrow_mut().prev = None;
                self.head = Some(new_head);
            } else {
                self.head = None;
                self.tail = None;
            }
            self.length -= 1;
            return;
        }
        let leader = self.traverse_to_index(index - 1).unwrap();
        let unwanted_node = leader.borrow().next.clone().unwrap();
        let follower = unwanted_node.borrow().next.clone();
        if let Some(follow) = follower {
            follow.borrow_mut().prev = Some(Rc::downgrade(&leader));
        } else {
            self.tail = Some(leader.clone());
        }
        leader.borrow_mut().next = follower;
        self.length -= 1;
    }

    /// Traverses the list to reach a node at a specific index.
    fn traverse_to_index(&self, index: usize) -> Option<Rc<RefCell<Node<T>>>> {
        let mut current = self.head.clone();
        let mut i = 0;
        while i < index {
            current = current.as_ref()?.borrow().next.clone();
            i += 1;
        }
        current
    }

    /// Prints the entire list and its length.
    pub fn printt(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} ", node.borrow().data);
            current = node.borrow().next.clone();
        }
        println!("\nLength {}", self.length);
    }
}

fn main() {
    let mut d = DoublyLinkedList::new();
    d.append(10);
    d.append(5);
    d.append(6);
    d.prepend(1);
    d.insert(2, 22);
    d.remove(3);
    d.printt();
}

// TODO: add tests