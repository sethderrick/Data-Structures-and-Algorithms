use std::clone::Clone;

/// A Node struct that stores a value and a link to the next node in the stack.
#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

/// A Stack struct that uses a linked list to store elements in a last-in, first-out (LIFO) order.
struct Stack<T> {
    top: Option<Box<Node<T>>>,
    bottom: Option<Box<Node<T>>>,
    length: usize,
}

impl<T: Clone> Stack<T> {
    /// Constructs a new, empty Stack.
    pub fn new() -> Self {
        Stack {
            top: None,
            bottom: None,
            length: 0,
        }
    }

    /// Returns a reference to the top element of the stack without removing it,
    /// if the stack is not empty.
    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.value)
    }

    /// Adds a new element to the top of the stack.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to push onto the stack.
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.top.take(),
        });

        self.top = Some(new_node);

        if self.bottom.is_none() {
            self.bottom = self.top.clone();
        }

        self.length += 1;
    }

    /// Removes the top element from the stack and returns it, if the stack is not empty.
    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            self.top = node.next;
            if self.length == 1 {
                self.bottom = None;
            }
            self.length -= 1;
            node.value
        })
    }

    /// Checks if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

fn main() {
    let mut my_stack = Stack::new();

    my_stack.push("google");
    println!("Stack after pushing 'google': {:?}", my_stack.peek());

    my_stack.push("google2");
    println!("Stack after pushing 'google2': {:?}", my_stack.peek());

    println!("Top of the stack: {:?}", my_stack.peek());

    my_stack.pop();
    println!("Stack after one pop: {:?}", my_stack.peek());

    my_stack.pop();
    println!("Stack after two pops: {:?}", my_stack.peek());
}
