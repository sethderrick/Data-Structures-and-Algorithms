/*
  Ownership and Borrowing: The use of Rc<RefCell<T>> allows multiple 
  ownership and mutable borrowing in the list, which is crucial for a 
  linked list structure in Rust because nodes need to be mutable even 
  if they are owned by multiple parts of the program.

  Error Handling: Instead of panicking when trying to peek an empty 
  queue, peek now safely returns Option<&T>, where None indicates the 
  queue is empty. This change aligns with Rust's emphasis on safety 
  and handling edge cases without crashing.

  Dequeue Method: It uses Rc::try_unwrap to take ownership of the value 
  inside the Rc if it's the last reference. This method provides a clean 
  way to remove the first node and properly manage the links between 
  nodes, especially handling the tail pointer when necessary.

  Memory Management: Rust's memory management is explicit and safe, 
  preventing memory leaks and dangling pointers through its ownership
   system, unlike other languages' garbage collector.
*/


use std::cell::RefCell;
use std::rc::Rc;

/// A node in the queue, holding a value and a pointer to the next node.
struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

/// A FIFO (first-in, first-out) data structure implemented with linked nodes.
pub struct Queue<T> {
    first: Option<Rc<RefCell<Node<T>>>>,
    last: Option<Rc<RefCell<Node<T>>>>,
    length: usize,
}

impl<T> Queue<T>
where
    T: std::fmt::Display, // Require T to implement the Display trait for printing.
{
    /// Constructs a new, empty `Queue`.
    pub fn new() -> Self {
        Queue {
            first: None,
            last: None,
            length: 0,
        }
    }

    /// Returns the value at the front of the queue without removing it.
    /// Panics if the queue is empty.
    pub fn peek(&self) -> Option<&T> {
        self.first.as_ref().map(|node| &node.borrow().val)
    }

    /// Adds a value to the end of the queue.
    pub fn enqueue(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            next: None,
        }));
        match self.last.take() {
            Some(old_last) => old_last.borrow_mut().next = Some(new_node.clone()),
            None => self.first = Some(new_node.clone()),
        };
        self.last = Some(new_node);
        self.length += 1;
    }

    /// Removes the value at the front of the queue and returns it.
    /// Returns `None` if the queue is empty.
    pub fn dequeue(&mut self) -> Option<T> {
        self.first.take().map(|old_first| {
            let old_first = Rc::try_unwrap(old_first).ok().unwrap().into_inner();
            self.first = old_first.next;
            if self.first.is_none() {
                self.last = None;
            }
            self.length -= 1;
            old_first.val
        })
    }

    /// Prints all values in the queue from front to back.
    pub fn printt(&self) {
        let mut current = self.first.clone();
        while let Some(node) = current {
            print!("{} -> ", node.borrow().val);
            current = node.borrow().next.clone();
        }
        println!("\nLength: {}", self.length);
    }
}

fn main() {
    let mut myq = Queue::new();
    myq.enqueue("google");
    myq.enqueue("microsoft");
    myq.enqueue("facebook");
    myq.enqueue("apple");
    myq.printt();
    myq.dequeue();
    myq.printt();
    if let Some(x) = myq.peek() {
        println!("{}", x);
    }
}

// TODO: add tests