#[derive(Debug)]
struct MyArray {
    length: usize,
    data: Vec<String>,
}

impl MyArray {
    fn new() -> Self {
        MyArray {
            length: 0,
            data: Vec::new(),
        }
    }

    fn get(&self, index: usize) -> Option<&String> {
        self.data.get(index)
    }

    fn push(&mut self, item: String) -> usize {
        self.data.push(item);
        self.length = self.data.len();
        self.length
    }

    fn pop(&mut self) -> Option<String> {
        self.data.pop()
    }

    fn delete(&mut self, index: usize) -> Option<String> {
        if index >= self.length {
            return None;
        }

        let item = self.data.remove(index);
        self.length = self.data.len();
        Some(item)
    }

    fn shift_items(&mut self, index: usize) {
        for i in index..self.length - 1 {
            self.data[i] = self.data[i + 1].clone();
        }

        self.data.pop();
        self.length = self.data.len();
    }
}

fn main() {
    let mut new_array = MyArray::new();

    new_array.push("hi".to_string());
    new_array.push("you".to_string());
    new_array.push("!".to_string());

    new_array.pop();

    new_array.delete(1);

    println!("{:?}", new_array);
}
