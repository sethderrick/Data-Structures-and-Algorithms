/*
  This example uses a singly linked list approach, similar to the JavaScript
  version in the originalaneagoie repo.

  There are several alternative approaches to implementing a queue in Rust that can
  offer different advantages in terms of efficiency, stability, or general usability,
  depending on the specific requirements of the application:

  Using a Vec (Dynamic Array):
    Description: Instead of a linked list, you can use Rust's Vec to implement a queue.
      You would push elements to the end of the vector and remove them from the beginning.
      However, removing elements from the front of a vector can be inefficient because it
      requires shifting all other elements.
    Advantages: This approach is memory-efficient and cache-friendly due to the contiguous
      allocation of elements, which can lead to better performance in scenarios where the queue
    size fluctuates less frequently.
    Disadvantages: Frequent insertions and deletions at the beginning of the vector can lead
      to poor performance due to the need to shift elements.

  Ring Buffer (Circular Queue):
    Description: A circular buffer uses a fixed-size array and two pointers (or indices) to
      keep track of the start and end of the queue. This setup allows for efficient addition and
      removal of elements at both ends of the queue without shifting elements.
    Advantages: Highly efficient for queues where the maximum size is known beforehand because
      it eliminates the overhead of memory allocation after the initial setup and avoids data
      shifting entirely.
    Disadvantages: The size of the queue is fixed, which means it cannot grow beyond the initially
      allocated size without a complex resizing mechanism.

  Using std::collections::VecDeque:
    Description: Rust’s standard library provides a double-ended queue implementation with VecDeque. It supports efficient push and pop operations from both ends.
    Advantages: VecDeque is implemented as a growable ring buffer, combining the advantages of
      dynamic resizing and efficient operations from both ends.
    Disadvantages: While generally efficient, VecDeque may still suffer from occasional performance
      hits during buffer resizing (less frequent than with a simple vector).

  Using third-party libraries like crossbeam-deque:
    Description: Some third-party libraries offer high-performance and concurrent data structures.
      For example, crossbeam-deque provides a work-stealing deque which is excellent for building schedulers.
    Advantages: Optimized for concurrent access, making it suitable for multi-threaded environments.
    Disadvantages: Introduces external dependencies and might be overkill for simple, single-threaded
      scenarios.

  I think, for most general applications, VecDeque provides a good balance between performance
  and ease of use.
*/

use std::clone::Clone;

// A Node struct that stores a value and a link to the next node in the queue.
#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

/// A Queue struct that uses a linked list to store elements in a first-in,
/// first-out (FIFO) order.
struct Queue<T> {
    first: Option<Box<Node<T>>>,
    last: Option<Box<Node<T>>>,
    length: usize,
}

impl<T: Clone> Queue<T> {
    /// Constructs a new, empty Queue.
    pub fn new() -> Self {
        Queue {
            first: None,
            last: None,
            length: 0,
        }
    }

    /// Returns a reference to the first element of the queue without removing it,
    /// if the queue is not empty.
    pub fn peek(&self) -> Option<&T> {
        self.first.as_ref().map(|node| &node.value)
    }

    /// Adds an element to the end of the queue.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the queue.
    pub fn enqueue(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });

        if let Some(ref mut last) = &mut self.last {
            last.next = Some(new_node.clone());
        } else {
            self.first = Some(new_node.clone());
        }

        self.last = Some(new_node);
        self.length += 1;
    }

    /// Removes the first element from the queue and returns it,
    /// if the queue is not empty.
    pub fn dequeue(&mut self) -> Option<T> {
        self.first.take().map(|node| {
            self.first = node.next;
            if self.first.is_none() {
                self.last = None;
            }
            self.length -= 1;
            node.value
        })
    }

    /// Checks if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

fn main() {
    let mut my_queue = Queue::new();
    my_queue.enqueue("Joy");
    my_queue.enqueue("Matt");
    my_queue.enqueue("Pavel");
    my_queue.enqueue("Samir");

    println!("Queue before dequeuing: {:?}", my_queue.peek());
    my_queue.dequeue();
    println!("Queue after dequeuing Joy: {:?}", my_queue.peek());

    println!("Current state of the queue:");
    while let Some(value) = my_queue.dequeue() {
        println!("{}", value);
    }
}
