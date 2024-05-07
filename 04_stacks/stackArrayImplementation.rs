/// A Stack struct that uses a Vec to store elements in a last-in,
/// first-out (LIFO) order.

struct Stack<T> {
    array: Vec<T>,
}

impl<T> Stack<T> {
    /// Constructs a new, empty Stack.
    pub fn new() -> Self {
        Stack { array: Vec::new() }
    }

    /// Returns a reference to the top element of the stack without removing it, if the stack is not empty.
    pub fn peek(&self) -> Option<&T> {
        self.array.last()
    }

    /// Adds an element to the top of the stack.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to push onto the stack.
    pub fn push(&mut self, value: T) {
        self.array.push(value);
    }

    /// Removes the top element from the stack and returns it, if the stack is not empty.
    pub fn pop(&mut self) -> Option<T> {
        self.array.pop()
    }
}

fn main() {
    let mut my_stack = Stack::new();
    println!("Initial top of stack: {:?}", my_stack.peek());

    my_stack.push("google");
    my_stack.push("ztm");
    my_stack.push("discord");
    println!("Top of stack after pushes: {:?}", my_stack.peek());

    my_stack.pop();
    println!("Top of stack after one pop: {:?}", my_stack.peek());
    my_stack.pop();
    println!("Top of stack after two pops: {:?}", my_stack.peek());
    my_stack.pop();
    println!("Top of stack after three pops: {:?}", my_stack.peek());
}
