use std::collections::HashMap;

// Simple function to add 80 to a number
fn add_to_80(n: i32) -> i32 {
    n + 80
}

// This version uses a global mutable cache which is not idiomatic Rust and can lead to safety issues
// in a concurrent context.
// Therefore, we use `unsafe` and a static mutable HashMap for demonstration purposes only and to align
// with the original JavaScript version in the aneagoie repo
static mut CACHE: HashMap<i32, i32> = HashMap::new();

fn memoize_add_to_80_v1(n: i32) -> i32 {
    unsafe {
        if let Some(&value) = CACHE.get(&n) {
            value
        } else {
            println!("long time");
            let answer = n + 80;
            CACHE.insert(n, answer);
            answer
        }
    }
}

// Second version using a closure to encapsulate the cache, avoiding global state
// This is more idiomatic in Rust and safer, avoiding global state. This function
// is encapsulated and manages its cache internally, similar to the original JavaScript closure.
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
