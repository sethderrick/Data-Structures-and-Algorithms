/*
  Key Points in the Rust Implementation:

  I may re-write using Rc and RefCell for reference-counted and mutable
  access to nodes. May also use for the prev pointers to avoid creating
  reference cycles.

  Rust's Option type is used extensively to handle the presence and absence
  of nodes, ensuring that accesses are checked for null values, which helps
  prevent runtime errors.
*/

/// A node in the doubly linked list, containing a value and pointers to the next and previous nodes.
struct Node<T> {
    value: T,
    prev: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
}

/// A doubly linked list with operations to append, prepend, insert, and remove elements.
struct DoublyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> DoublyLinkedList<T>
where
    T: Clone,
{
    /// Creates a new doubly linked list with a single initial element.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value to insert into the list.
    pub fn new(value: T) -> Self {
        let node = Box::new(Node {
            value,
            prev: None,
            next: None,
        });
        DoublyLinkedList {
            head: Some(node.clone()),
            tail: Some(node),
            length: 1,
        }
    }

    /// Appends a value to the end of the list.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to append.
    pub fn append(&mut self, value: T) {
        let mut new_node = Box::new(Node {
            value,
            prev: None,
            next: None,
        });
        match self.tail.take() {
            Some(mut old_tail) => {
                new_node.prev = Some(old_tail.clone());
                old_tail.next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.length += 1;
    }

    /// Prepends a value to the beginning of the list.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to prepend.
    pub fn prepend(&mut self, value: T) {
        let mut new_node = Box::new(Node {
            value,
            prev: None,
            next: None,
        });
        match self.head.take() {
            Some(old_head) => {
                new_node.next = Some(old_head.clone());
                old_head.prev = Some(new_node.clone());
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.length += 1;
    }

    /// Inserts a value at a specified index in the list.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index at which to insert the value.
    /// * `value` - The value to insert.
    pub fn insert(&mut self, index: usize, value: T) {
        if index >= self.length {
            self.append(value);
            return;
        }
        let mut new_node = Box::new(Node {
            value,
            prev: None,
            next: None,
        });
        if index == 0 {
            self.prepend(value);
            return;
        }
        let mut cursor = self.head.clone();
        for _ in 0..index - 1 {
            cursor = cursor.and_then(|mut node| node.next.take());
        }
        if let Some(mut leader) = cursor {
            let follower = leader.next.take();
            new_node.prev = Some(leader.clone());
            new_node.next = follower.clone();
            if let Some(mut follower) = follower {
                follower.prev = Some(new_node.clone());
                leader.next = Some(new_node.clone());
                self.length += 1;
            }
        }
    }

    /// Removes a node from the list at a specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the node to remove.
    pub fn remove(&mut self, index: usize) {
        if index >= self.length {
            return;
        }
        if index == 0 {
            self.head = self.head.take().and_then(|node| node.next);
            if let Some(ref mut head) = self.head {
                head.prev.take();
            }
            self.length -= 1;
            return;
        }
        let mut cursor = self.head.clone();
        for _ in 0..index - 1 {
            cursor = cursor.and_then(|mut node| node.next.take());
        }
        if let Some(mut leader) = cursor {
            let unwanted_node = leader.next.take();
            leader.next = unwanted_node.and_then(|mut node| node.next.take());
            if let Some(ref mut new_next) = leader.next {
                new_next.prev = Some(leader.clone());
            }
            self.length -= 1;
        }
    }

    /// Prints the list to the console.
    pub fn print_list(&self) {
        let mut elements = Vec::new();
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            elements.push(node.value.clone());
            current = node.next.as_ref();
        }
        println!("{:?}", elements);
    }
}

fn main() {
    let mut my_linked_list = DoublyLinkedList::new(10);
    my_linked_list.append(5);
    my_linked_list.append(16);
    my_linked_list.prepend(1);
    my_linked_list.print_list();
    my_linked_list.insert(2, 99);
    my_linked_list.remove(2);
    my_linked_list.print_list();
}
