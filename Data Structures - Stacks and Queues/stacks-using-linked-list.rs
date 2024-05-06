/* 
  
*/


use std::cell::RefCell;
use std::rc::Rc;

/// A node in the stack, holding some data and a pointer to the next node.
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

/// A stack implementation using a singly linked list.
pub struct Stack<T> {
    top: Option<Rc<RefCell<Node<T>>>>,
    bottom: Option<Rc<RefCell<Node<T>>>>,
    length: usize,
}

impl<T> Stack<T>
where
    T: std::fmt::Display + std::fmt::Debug, // Display for printing, Debug for debugging
{
    /// Constructs a new, empty `Stack`.
    pub fn new() -> Self {
        Stack {
            top: None,
            bottom: None,
            length: 0,
        }
    }

    /// Returns the top element of the stack without removing it.
    /// Returns `None` if the stack is empty.
    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.borrow().data)
    }

    /// Adds a new element to the top of the stack.
    pub fn push(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: self.top.clone(),
        }));
        if self.bottom.is_none() {
            self.bottom = Some(new_node.clone());
        }
        self.top = Some(new_node);
        self.length += 1;
    }

    /// Removes the top element of the stack and returns it.
    /// Returns `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|top_node| {
            let top_node = Rc::try_unwrap(top_node).ok().unwrap().into_inner();
            self.top = top_node.next;
            if self.length == 1 {
                self.bottom = None;
            }
            self.length -= 1;
            top_node.data
        })
    }

    /// Prints all elements from top to bottom.
    pub fn printt(&self) {
        let mut current = self.top.clone();
        while let Some(node) = current {
            print!("{} -> ", node.borrow().data);
            current = node.borrow().next.clone();
        }
        println!();
    }
}

fn main() {
    let mut mystack = Stack::new();
    mystack.push("google");
    mystack.push("microsoft");
    mystack.push("facebook");
    mystack.push("apple");
    mystack.printt();
    if let Some(x) = mystack.peek() {
        println!("{}", x);
    }
    if let Some(y) = mystack.pop() {
        println!("{}", y);
    }
    mystack.printt();
    if let Some(qw) = mystack.peek() {
        println!("{}", qw);
    }
}

// TODO: add tests