/*
    Rust doesn't have built-in support for function closures
    retaining mutable state but you can achieve similar functionality
    using structs or by using Cell or RefCell for internal mutability.
*/
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn add80(n: i32) -> i32 {
    println!("Long time");
    n + 80
}

fn main() {
    println!("{}", add80(5));
    println!("{}", add80(5));
}

// Memoization 1
struct MemoizedAdd80 {
    cache: HashMap<i32, i32>,
}

impl MemoizedAdd80 {
    fn new() -> Self {
        MemoizedAdd80 {
            cache: HashMap::new(),
        }
    }

    fn call(&mut self, n: i32) -> i32 {
        if let Some(&value) = self.cache.get(&n) {
            value
        } else {
            println!("Long time");
            let result = n + 80;
            self.cache.insert(n, result);
            result
        }
    }
}

// Memoization 2
fn memoized_add80_factory() -> impl Fn(i32) -> i32 {
    let cache = Rc::new(RefCell::new(HashMap::new()));
    move |n: i32| {
        let mut cache_borrowed = cache.borrow_mut();
        if let Some(&value) = cache_borrowed.get(&n) {
            value
        } else {
            println!("Long time");
            let result = n + 80;
            cache_borrowed.insert(n, result);
            result
        }
    }
}

fn main() {
    // M1
    let mut memoized_add80 = MemoizedAdd80::new();
    println!("{}", memoized_add80.call(6));
    println!("{}", memoized_add80.call(6));

    // M2
    let memo = memoized_add80_factory();
    println!("{}", memo(7));
    println!("{}", memo(7));
}

// TODO: add tests
