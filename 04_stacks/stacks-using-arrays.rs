/*
  Stack Operations: The stack supports typical operations:
    • push to add an element to the top.
    • pop to remove the top element and return it, safely returning 
      None when the stack is empty.
    • peek to view the top element without removing it, also returning 
      None if the stack is empty.
  
  Type Safety: The Stack struct is generic over type T, which means 
  it can hold elements of any type

  Debug Implementation: The impl<T: std::fmt::Debug> std::fmt::Debug 
  for Stack<T> block allows the stack to be printed using the debug 
  format, which is useful for checking the internal state during 
  development or debugging.

  Memory Management: Rust's Vec manages memory automatically and efficiently, 
  growing and shrinking as elements are pushed and popped. This management 
  is safe and prevents common errors such as buffer overflow.
*/


/// A simple stack implementation using a vector to store elements.
pub struct Stack<T> {
    arr: Vec<T>,
}

impl<T> Stack<T> {
    /// Constructs a new, empty `Stack`.
    pub fn new() -> Self {
        Stack { arr: Vec::new() }
    }

    /// Returns the top element of the stack without removing it.
    /// Returns `None` if the stack is empty.
    pub fn peek(&self) -> Option<&T> {
        self.arr.last()
    }

    /// Adds an element to the top of the stack.
    pub fn push(&mut self, value: T) {
        self.arr.push(value);
    }

    /// Removes the top element of the stack and returns it.
    /// Returns `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.arr.pop()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Stack<T> {
    /// Provides a way to format the stack for printing, displaying its internal state.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stack")
         .field("arr", &self.arr)
         .finish()
    }
}

fn main() {
    let mut mystack = Stack::new();
    mystack.push("google");
    mystack.push("microsoft");
    mystack.push("facebook");
    mystack.push("apple");

    // Print the entire stack
    println!("{:?}", mystack);

    // Peek the top element
    if let Some(x) = mystack.peek() {
        println!("{}", x);
    }

    // Pop the top element
    mystack.pop();

    // Print the stack after popping
    println!("{:?}", mystack);
}

// TODO: add tests