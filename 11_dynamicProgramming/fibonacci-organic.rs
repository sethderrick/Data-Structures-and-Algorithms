use std::collections::HashMap;

/// Computes the Fibonacci number at the nth position using memoization to avoid redundant calculations.
/// A mutable reference to a hash map is used to store previously computed Fibonacci numbers.
///
/// Arguments:
/// * `n`: the position in the Fibonacci sequence.
/// * `cache`: a mutable reference to a HashMap for storing and retrieving previously computed values.
///
/// Returns:
/// The Fibonacci number at the nth position.
fn fib(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    match cache.get(&n) {
        Some(&value) => return value,
        None => {
            if n < 2 {
                cache.insert(n, n);
                return n;
            }

            let result = fib(n - 1, cache) + fib(n - 2, cache);
            cache.insert(n, result);
            return result;
        }
    }
}

fn main() {
    let mut cache = HashMap::new();  // Cache initialization.
    let result = fib(10, &mut cache);  // Calculating Fibonacci of 10.

    println!("Fibonacci(10) = {}", result);
    println!("Cache state: {:?}", cache);
}

// TODO: add tests