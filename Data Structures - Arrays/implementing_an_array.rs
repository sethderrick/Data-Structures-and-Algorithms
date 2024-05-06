struct Array<T> {
    data: Vec<T>,
}

impl<T> Array<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn get(&self, index: usize) -> &T {
        &self.data[index]
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn delete(&mut self, index: usize) -> T {
        let result = self.data.remove(index);
        result
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Array")
         .field("data", &self.data)
         .finish()
    }
}

fn main() {
    let mut arr = Array::new();
    arr.push(3);
    arr.push("hi");
    arr.push(34);
    arr.push(20);
    arr.push("hey");
    arr.push("welcome");
    arr.delete(3);

    println!("{:?}", arr);
}

// TODO: Add tests