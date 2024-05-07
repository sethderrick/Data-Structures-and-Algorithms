/*
    Write two functions that finds the factorial of any number. One should use recursive, the other should just use a for loop

    function findFactorialRecursive(number) {
        //code here
        return answer;
    }

    function findFactorialIterative(number) {
        //code here
        return answer;
    }
*/

// ANSWER:

/// Calculates the factorial of a number using a recursive approach.
///
/// # Arguments
///
/// * `number` - The number whose factorial is to be calculated.
///
/// # Returns
///
/// The factorial of the given number.
///
/// # Examples
///
/// ```
/// let result = find_factorial_recursive(5);
/// assert_eq!(result, 120);
/// ```
fn find_factorial_recursive(number: u64) -> u64 {
    if number <= 1 {
        1 // Base case for 0! and 1!
    } else {
        number * find_factorial_recursive(number - 1)
    }
}

/// Calculates the factorial of a number using an iterative approach.
///
/// # Arguments
///
/// * `number` - The number whose factorial is to be calculated.
///
/// # Returns
///
/// The factorial of the given number.
///
/// # Examples
///
/// ```
/// let result = find_factorial_iterative(5);
/// assert_eq!(result, 120);
/// ```
fn find_factorial_iterative(number: u64) -> u64 {
    let mut answer = 1;
    for i in 2..=number {
        answer *= i;
    }
    answer
}

fn main() {
    let recursive_result = find_factorial_recursive(5);
    let iterative_result = find_factorial_iterative(5);

    println!("Recursive factorial of 5: {}", recursive_result);
    println!("Iterative factorial of 5: {}", iterative_result);
}
