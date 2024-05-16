use std::collections::HashMap;
use std::sync::{Mutex, Once};

// Simple function to add 80 to a number
fn add_to_80(n: i32) -> i32 {
    n + 80
}

// Use `Once` and `Mutex` for safe static initialization
static mut CACHE: Option<Mutex<HashMap<i32, i32>>> = None;
static INIT: Once = Once::new();

fn get_cache() -> &'static Mutex<HashMap<i32, i32>> {
    unsafe {
        INIT.call_once(|| {
            CACHE = Some(Mutex::new(HashMap::new()));
        });
        CACHE.as_ref().unwrap()
    }
}

fn memoize_add_to_80_v1(n: i32) -> i32 {
    let cache = get_cache();
    let mut cache_lock = cache.lock().unwrap();
    if let Some(&value) = cache_lock.get(&n) {
        value
    } else {
        println!("long time");
        let answer = n + 80;
        cache_lock.insert(n, answer);
        answer
    }
}

// Second version using a closure to encapsulate the cache, avoiding global state
fn memoize_add_to_80_v2() -> Box<dyn FnMut(i32) -> i32> {
    let mut cache = HashMap::new();
    Box::new(move |n| {
        if let Some(&value) = cache.get(&n) {
            value
        } else {
            println!("long time");
            let answer = n + 80;
            cache.insert(n, answer);
            answer
        }
    })
}

fn main() {
    println!("Add to 80: {}", add_to_80(5));

    // Example usage of the first version
    println!("1: {}", memoize_add_to_80_v1(6));
    // println!("cache {:?}", unsafe { &CACHE }); // Uncomment to view cache state
    // println!("-----------");
    println!("2: {}", memoize_add_to_80_v1(6));

    // Example usage of the second version
    let mut memoized = memoize_add_to_80_v2();
    println!("1: {}", memoized(6));
    // println!("cache"); // Can't directly view the cache here as it's encapsulated
    // println!("-----------");
    println!("2: {}", memoized(6));
}
