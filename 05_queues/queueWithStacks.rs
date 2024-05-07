/// A `CrazyQueue` struct that uses two vectors to implement a queue-like behavior.
/// The two vectors, `first` and `last`, are used to reverse the order of elements
/// to simulate enqueueing and dequeueing operations.
struct CrazyQueue<T> {
    first: Vec<T>,
    last: Vec<T>,
}

impl<T> CrazyQueue<T> {
    /// Constructs a new, empty `CrazyQueue`.
    pub fn new() -> Self {
        CrazyQueue {
            first: Vec::new(),
            last: Vec::new(),
        }
    }

    /// Adds an element to the queue.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the queue.
    pub fn enqueue(&mut self, value: T) {
        let length = self.first.len();

        for _ in 0..length {
            if let Some(val) = self.first.pop() {
                self.last.push(val);
            }
        }

        self.last.push(value);
    }

    /// Removes the first element from the queue.
    pub fn dequeue(&mut self) {
        let length = self.last.len();

        for _ in 0..length {
            if let Some(val) = self.last.pop() {
                self.first.push(val);
            }
        }

        self.first.pop(); // Discard the element being dequeued.
    }

    /// Returns a reference to the first element of the queue without removing it, if the queue is not empty.
    pub fn peek(&self) -> Option<&T> {
        if !self.first.is_empty() {
            return self.first.last();
        }

        self.last.first()
    }
}

fn main() {
    let mut my_queue = CrazyQueue::new();
    println!("Initial peek: {:?}", my_queue.peek());
    my_queue.enqueue("Joy");
    my_queue.enqueue("Matt");
    my_queue.enqueue("Pavel");
    println!("Peek after enqueues: {:?}", my_queue.peek());

    println!("========");
    my_queue.dequeue();
    println!("Peek after one dequeue: {:?}", my_queue.peek());
    my_queue.dequeue();
    println!("Peek after two dequeues: {:?}", my_queue.peek());
    my_queue.dequeue();
    println!("========");
    println!("Final peek: {:?}", my_queue.peek());
}
