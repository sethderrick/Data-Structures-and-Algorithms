/// Calculates the Fibonacci number at a given position using an
/// iterative approach.
///
/// Arguments:
/// * `num`: The position in the Fibonacci sequence.
///
/// Returns:
/// The Fibonacci number at the specified position.
fn fib(num: u32) -> u32 {
    if num < 2 {
        return num;
    }

    let mut a = 0;
    let mut b = 1;
    let mut total = 0;

    for _ in 1..num {
        total = a + b;
        a = b;
        b = total;
    }
    total
}

/// Calculates the Fibonacci number at a given position using a
/// recursive approach.
///
/// Arguments:
/// * `num`: The position in the Fibonacci sequence.
///
/// Returns:
/// The Fibonacci number at the specified position.
fn fibonacci(num: u32) -> u32 {
    if num < 2 {
        return num;
    }
    fib(num - 1) + fib(num - 2) // This can be very inefficient without memoization
}

fn main() {
    let range = 10;
    let fibs: Vec<u32> = (0..range).map(fib).collect();
    println!("Iterative Fibonacci sequence up to 10 terms: {:?}", fibs);

    let fibonaccis: Vec<u32> = (0..range).map(fibonacci).collect();
    println!(
        "Recursive Fibonacci sequence up to 10 terms: {:?}",
        fibonaccis
    );

    println!(
        "Expected sequence: {:?}",
        vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    );
}

// TODO: add tests
