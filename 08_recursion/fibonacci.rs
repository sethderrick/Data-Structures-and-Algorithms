/*
    Given a number N return the index value of the Fibonacci sequence,
    where the sequence is:

    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144 ...

    The pattern of the sequence is that each value is the sum of the 2 previous values,
    that means that for N=5 → 2+3

    For example: fibonacciRecursive(6) should return 8
*/
/// Calculates the nth Fibonacci number using an iterative approach.
///
/// # Arguments
///
/// * `n` - The position in the Fibonacci sequence.
///
/// # Returns
///
/// The nth Fibonacci number.
///
/// # Examples
///
/// ```
/// let result = fibonacci_iterative(3);
/// assert_eq!(result, 2); // The third Fibonacci number is 2
/// ```
fn fibonacci_iterative(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let mut arr = vec![0, 1]; // Initialize the first two Fibonacci numbers
    for i in 2..=n {
        arr.push(arr[i - 2] + arr[i - 1]); // Calculate the next Fibonacci number and push to the array
    }
    arr[n] // Return the nth Fibonacci number
}

/// Calculates the nth Fibonacci number using a recursive approach.
///
/// # Arguments
///
/// * `n` - The position in the Fibonacci sequence.
///
/// # Returns
///
/// The nth Fibonacci number.
///
/// # Examples
///
/// ```
/// let result = fibonacci_recursive(6);
/// assert_eq!(result, 8); // The sixth Fibonacci number is 8
/// ```
fn fibonacci_recursive(n: usize) -> usize {
    if n < 2 {
        return n; // Base case: return n when n is 0 or 1
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2) // Recursive case
}

fn main() {
    // Testing the iterative function
    let result_iterative = fibonacci_iterative(3);
    println!("Iterative Fibonacci of 3: {}", result_iterative);

    // Testing the recursive function
    let result_recursive = fibonacci_recursive(6);
    println!("Recursive Fibonacci of 6: {}", result_recursive);
}
