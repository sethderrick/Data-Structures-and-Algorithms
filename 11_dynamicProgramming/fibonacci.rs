use std::collections::HashMap;

// Global mutable state to keep track of the number of calculations, simulating the JavaScript example.
static mut CALCULATIONS: u32 = 0;

/*
    The JavaScript version of the fibonacci function is a straightforward recursive implementation
    without memoization, which indeed has a time complexity of 𝑂(2𝑛) because each call generates
    two more calls, except for the base cases.

    This same structure and behavior are precisely mirrored here in the Rust version.
*/
fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

/*
    The fibonacciMaster function in the original JavaScript used memoization (caching results to avoid
    redundant calculations), the complexity is 𝑂(𝑛). This is because each Fibonacci number from 0
    to n is calculated exactly once and then stored, making subsequent requests for already computed
    Fibonacci numbers constant time operations.

    In Rust, the equivalent function fibonacci_master achieves this using a closure that captures a mutable
    HashMap for caching
*/
fn fibonacci_master() -> Box<dyn FnMut(u32) -> u32> {
    let mut cache = HashMap::new();
    Box::new(move |n| {
        // Unsafe block to increment the global mutable state.
        unsafe {
            CALCULATIONS += 1;
        }
        if let Some(&value) = cache.get(&n) {
            value
        } else {
            if n < 2 {
                n
            } else {
                let result = fibonacci(n - 1) + fibonacci(n - 2);
                cache.insert(n, result);
                result
            }
        }
    })
}

/// Iterative Fibonacci function to generate the nth Fibonacci number.
fn fibonacci_master2(n: u32) -> u32 {
    let mut answer = vec![0, 1];
    for i in 2..=n {
        answer.push(answer[i as usize - 2] + answer[i as usize - 1]);
    }
    *answer.last().unwrap()
}

fn main() {
    let mut faster_fib = fibonacci_master();

    println!("Slow: {}", fibonacci(35));
    println!("DP: {}", faster_fib(100));
    println!("DP2: {}", fibonacci_master2(100));
    unsafe {
        println!("We did {} calculations", CALCULATIONS);
    }
}
