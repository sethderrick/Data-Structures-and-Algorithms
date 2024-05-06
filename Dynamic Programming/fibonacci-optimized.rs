/// don't forget to add 'lru' to your Cargo.toml
/// Ex:   [dependencies]
///       lru = "0.7"
use lru::LruCache;

/// Fibonacci function using an LRU cache to store and retrieve previously computed values.
fn fib(n: u64, cache: &mut LruCache<u64, u64>) -> u64 {
    if let Some(&value) = cache.get(&n) {
        return value;
    }

    let result = if n < 2 {
        n
    } else {
        fib(n - 1, cache) + fib(n - 2, cache)
    };

    cache.put(n, result);
    result
}

fn main() {
    let mut cache = LruCache::new(1000);
    println!("{}", fib(10, &mut cache));
    println!("{:?}", cache); // This prints the current state of the LRU cache.
}

// TODO: add tests