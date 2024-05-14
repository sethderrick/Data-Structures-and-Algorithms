use std::fmt::Debug;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>, // Using raw pointer for mutability
    length: usize,
}

impl<T: Clone + Debug> LinkedList<T> {
    fn new(value: T) -> Self {
        let new_node = Box::new(Node { value, next: None });
        let raw_node: *mut _ = Box::into_raw(new_node.clone());

        LinkedList {
            head: Some(new_node),
            tail: Some(raw_node),
            length: 1,
        }
    }

    fn append(&mut self, value: T) {
        let mut new_node = Box::new(Node { value, next: None });
        let raw_node: *mut _ = &mut *new_node;

        if let Some(tail) = self.tail {
            unsafe {
                (*tail).next = Some(new_node);
            }
        } else {
            self.head = Some(new_node.clone());
        }

        self.tail = Some(raw_node);
        self.length += 1;
    }

    fn prepend(&mut self, value: T) {
        let mut new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        let raw_node: *mut _ = &mut *new_node;
        self.head = Some(new_node);

        if self.tail.is_none() {
            self.tail = Some(raw_node);
        }

        self.length += 1;
    }

    fn insert(&mut self, index: usize, value: T) {
        if index >= self.length {
            return self.append(value);
        }

        if index == 0 {
            return self.prepend(value);
        }

        let mut current = self.head.as_mut();
        for _ in 0..index - 1 {
            if let Some(current_node) = current {
                current = current_node.next.as_mut();
            } else {
                return;
            }
        }

        if let Some(current_node) = current {
            let mut new_node = Box::new(Node {
                value,
                next: current_node.next.take(),
            });
            current_node.next = Some(new_node);
            self.length += 1;
        }
    }

    fn remove(&mut self, index: usize) {
        if index >= self.length {
            return;
        }

        if index == 0 {
            if let Some(mut head_node) = self.head.take() {
                self.head = head_node.next.take();

                if self.length == 1 {
                    self.tail = None;
                }
                self.length -= 1;
                return;
            }
        }

        let mut current = self.head.as_mut();
        for _ in 0..index - 1 {
            if let Some(current_node) = current {
                current = current_node.next.as_mut();
            } else {
                return;
            }
        }

        if let Some(current_node) = current {
            let mut node_to_remove = current_node.next.take();
            if let Some(ref mut next_node) = node_to_remove {
                current_node.next = next_node.next.take();

                if index == self.length - 1 {
                    self.tail = Some(Box::into_raw(next_node.clone()));
                }
            }

            self.length -= 1;
        }
    }

    fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = &self.head;

        while let Some(ref node) = current {
            result.push(node.value.clone());
            current = &node.next;
        }

        result
    }
}

fn main() {
    let mut my_linked_list = LinkedList::new(10);
    my_linked_list.append(5);
    my_linked_list.append(16);
    my_linked_list.prepend(1);
    my_linked_list.insert(2, 99);
    my_linked_list.insert(20, 88);
    my_linked_list.remove(2);
    println!("{:?}", my_linked_list.to_vec());
}
