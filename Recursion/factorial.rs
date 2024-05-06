/// Computes the factorial of a given number using recursion.
///
/// Arguments:
/// * `num`: The positive integer for which the factorial is computed.
///
/// Returns:
/// The factorial of `num`.
///
/// # Examples
/// ```
/// let result = factorial(4);
/// assert_eq!(result, 24);
/// ```
fn factorial(num: u32) -> u32 {
    if num == 1 {
        1
    } else {
        num * factorial(num - 1)
    }
}

/// Computes the factorial of a given number using an iterative approach.
///
/// Arguments:
/// * `num`: The positive integer for which the factorial is computed.
///
/// Returns:
/// The factorial of `num`.
///
/// # Examples
/// ```
/// let result = fact(6);
/// assert_eq!(result, 720);
/// ```
fn fact(num: u32) -> u32 {
    let mut result = 1;
    for i in 1..=num {
        result *= i;
    }
    result
}

fn main() {
    let result = factorial(4);
    println!("Factorial (Recursive) of 4 is: {}", result);

    let result = fact(6);
    println!("Factorial (Iterative) of 6 is: {}", result);
}

// TODO: add tests
