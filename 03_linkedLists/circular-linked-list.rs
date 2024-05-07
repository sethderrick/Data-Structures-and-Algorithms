/*
  Allows for multiple ownership and mutation even behind shared references, 
  which is essential for a circular structure where nodes may be both ahead 
  and behind each other.
*/

use std::cell::RefCell;
use std::rc::Rc;

/// A node in a circular linked list, holding a value and a pointer to the next node.
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

/// A circular linked list with operations to manipulate nodes at different positions.
pub struct CircularLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T> CircularLinkedList<T>
where
    T: std::fmt::Display + Copy, // T must support Display and Copy traits.
{
    /// Creates a new empty `CircularLinkedList`.
    pub fn new() -> Self {
        CircularLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    /// Checks if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Adds a node with the specified value at the beginning of the list.
    pub fn add_first(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data: value,
            next: self.head.clone(),
        }));
        if self.is_empty() {
            self.tail = Some(new_node.clone());
        } else {
            self.tail.as_ref().unwrap().borrow_mut().next = Some(new_node.clone());
        }
        self.head = Some(new_node);
        self.size += 1;
    }

    /// Adds a node with the specified value at the end of the list.
    pub fn add_last(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data: value,
            next: self.head.clone(),
        }));
        if self.is_empty() {
            self.head = Some(new_node.clone());
        } else {
            self.tail.as_ref().unwrap().borrow_mut().next = Some(new_node.clone());
        }
        self.tail = Some(new_node);
        self.size += 1;
    }

    /// Adds a node with the specified value at a particular position in the list.
    /// Positioning is 1-based; inserting at position 1 will add the node as the first element.
    pub fn add_particular(&mut self, value: T, position: usize) {
        if position > self.size + 1 || position == 0 {
            println!("Invalid position");
            return;
        }
        if position == 1 {
            self.add_first(value);
            return;
        }
        let new_node = Rc::new(RefCell::new(Node {
            data: value,
            next: None,
        }));
        let mut current = self.head.as_ref().unwrap().clone();
        for _ in 1..position - 1 {
            current = current.borrow().next.as_ref().unwrap().clone();
        }
        new_node.borrow_mut().next = current.borrow().next.clone();
        current.borrow_mut().next = Some(new_node);
        if position == self.size + 1 {
            self.tail = Some(current.borrow().next.clone().unwrap());
        }
        self.size += 1;
    }

    /// Removes and returns the first element of the list. Returns `None` if the list is empty.
    pub fn remove_first(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let old_head = old_head.borrow();
            if self.size == 1 {
                self.tail = None;
            } else {
                let new_head = old_head.next.as_ref().unwrap().clone();
                self.tail.as_ref().unwrap().borrow_mut().next = Some(new_head.clone());
                self.head = Some(new_head);
            }
            self.size -= 1;
            old_head.data
        })
    }

    /// Removes and returns the last element of the list. Returns `None` if the list is empty.
    pub fn remove_last(&mut self) -> Option<T> {
        if self.size == 1 {
            return self.remove_first();
        }
        let mut current = self.head.as_ref().unwrap().clone();
        while current.borrow().next.as_ref().unwrap() != self.tail.as_ref().unwrap() {
            current = current.borrow().next.clone().unwrap();
        }
        let old_tail = self.tail.take().unwrap();
        self.tail = Some(current.clone());
        current.borrow_mut().next = self.head.clone();
        self.size -= 1;
        Some(old_tail.borrow().data)
    }

    /// Prints the list's data in order from head to tail.
    pub fn print_list(&self) {
        let mut current = self.head.clone();
        let mut count = 0;
        while count < self.size {
            let current_ref = current.as_ref().unwrap().borrow();
            print!("{} --> ", current_ref.data);
            current = current_ref.next.clone();
            count += 1;
        }
        println!("end");
    }
}

fn main() {
    let mut circ_list = CircularLinkedList::new();
    circ_list.add_first(1);
    circ_list.add_last(2);
    circ_list.add_last(3);
    circ_list.add_last(5);
    circ_list.add_particular(4, 4);
    circ_list.print_list();
    println!("Removed: {:?}", circ_list.remove_first());
    circ_list.print_list();
    println!("Removed: {:?}", circ_list.remove_last());
    circ_list.print_list();
    println!("Removed: {:?}", circ_list.remove_particular(2));
    circ_list.print_list();
}

// TODO: add tests