/*
    Leverageing Rust's ownership model to safely manage memory without manual
    intervention.

    The LinkedList can store any type of data that supports the Clone and
    Debug traits.

    The to_vec method is a utility for converting the list to a vector for
    easy printing or inspection.
*/

/// A node in the singly linked list, containing a value and an optional next node.
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

/// A singly linked list with basic operations such as append, prepend, insert, remove, and reverse.
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T>
where
    T: Clone + std::fmt::Debug,
{
    /// Creates a new singly linked list with an initial value.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value to be placed in the list.
    pub fn new(value: T) -> Self {
        let node = Box::new(Node { value, next: None });
        LinkedList {
            head: Some(node.clone()),
            tail: Some(node),
            length: 1,
        }
    }

    /// Appends a value to the end of the list.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to append to the list.
    pub fn append(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });
        if let Some(ref mut tail) = self.tail {
            tail.next = Some(new_node);
        }
        self.tail = self
            .head
            .as_mut()
            .and_then(|node| node.next.as_mut())
            .map(|node| node as &mut Box<Node<T>>);
        self.length += 1;
    }

    /// Prepends a value to the beginning of the list.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to prepend.
    pub fn prepend(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        if self.length == 0 {
            self.tail = self.head.as_mut().map(|node| node as &mut Box<Node<T>>);
        }
        self.length += 1;
    }

    /// Inserts a value at a specified index in the list.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index where the value should be inserted.
    /// * `value` - The value to insert.
    pub fn insert(&mut self, index: usize, value: T) {
        if index == 0 {
            self.prepend(value);
            return;
        }
        if index >= self.length {
            self.append(value);
            return;
        }

        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            current = &mut current.as_mut().unwrap().next;
        }
        let new_node = Box::new(Node {
            value,
            next: current.as_mut().unwrap().next.take(),
        });
        current.as_mut().unwrap().next = Some(new_node);
        self.length += 1;
    }

    /// Removes a node from the list at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the node to remove.
    pub fn remove(&mut self, index: usize) {
        if index == 0 {
            self.head = self.head.take().and_then(|node| node.next);
            self.length -= 1;
            return;
        }
        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            current = &mut current.as_mut().unwrap().next;
        }
        let next_node = current.as_mut().unwrap().next.take().unwrap().next;
        current.as_mut().unwrap().next = next_node;
        if index == self.length - 1 {
            self.tail = Some(current.as_mut().unwrap().clone());
        }
        self.length -= 1;
    }

    /// Reverses the linked list in place.
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            current = next;
        }
        self.head = prev;
    }

    /// Converts the list into a vector of its elements for easy printing or further processing.
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut current = &self.head;
        while let Some(ref node) = current {
            vec.push(node.value.clone());
            current = &node.next;
        }
        vec
    }
}

fn main() {
    let mut my_linked_list = LinkedList::new(10);
    my_linked_list.append(5);
    my_linked_list.append(16);
    my_linked_list.prepend(1);
    println!("List before operations: {:?}", my_linked_list.to_vec());
    my_linked_list.insert(2, 99);
    my_linked_list.insert(20, 88);
    println!("List after insertions: {:?}", my_linked_list.to_vec());
    my_linked_list.remove(2);
    println!("List after removal: {:?}", my_linked_list.to_vec());
    my_linked_list.reverse();
    println!("List after reversing: {:?}", my_linked_list.to_vec());
}
