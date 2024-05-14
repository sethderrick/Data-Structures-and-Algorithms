use std::fmt::Debug;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>, // Added previous node pointer for a doubly linked list
}

struct DoubleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

impl<T: Clone + Debug> DoubleLinkedList<T> {
    fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, value: T) {
        let mut new_node = Box::new(Node {
            value,
            next: self.head.clone(),
            prev: None,
        });

        if let Some(ref mut old_head) = self.head {
            old_head.prev = Some(new_node.clone());
        } else {
            self.tail = Some(new_node.clone());
        }

        self.head = Some(new_node);
    }

    fn push_back(&mut self, value: T) {
        let mut new_node = Box::new(Node {
            value,
            next: None,
            prev: self.tail.clone(),
        });

        if let Some(ref mut old_tail) = self.tail {
            old_tail.next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }

        self.tail = Some(new_node);
    }

    fn print_list(&self) {
        let mut elements = Vec::new();
        let mut cursor = self.head.clone();

        while let Some(node) = cursor {
            elements.push(node.value);
            cursor = node.next.clone();
        }

        println!("{:?}", elements);
    }
}

fn main() {
    let mut list = DoubleLinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_back(3);

    println!("List elements:");
    list.print_list();
}
