#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>, // Using raw pointer for mutability
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, value: T) {
        let mut new_node = Box::new(Node { value, next: None });

        let raw_node: *mut _ = &mut *new_node;

        if self.tail.is_some() {
            unsafe {
                (*self.tail.unwrap()).next = Some(new_node);
            }
        } else {
            self.head = Some(new_node);
        }

        self.tail = Some(raw_node);
    }

    fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut current_node) = current {
            let next = current_node.next.take();
            current_node.next = prev;
            prev = Some(current_node);
            current = next;
        }

        self.head = prev;

        self.tail = self
            .head
            .as_deref_mut()
            .map(|node| {
                let mut last = node;
                while let Some(ref mut next_node) = last.next {
                    last = next_node;
                }
                last
            })
            .map(|last| last as *mut _);
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    println!("Original list:");
    let mut current = &list.head;
    while let Some(ref node) = current {
        println!("{}", node.value);
        current = &node.next;
    }

    list.reverse();

    println!("Reversed list:");
    let mut current = &list.head;
    while let Some(ref node) = current {
        println!("{}", node.value);
        current = &node.next;
    }
}
