/*
  Given 2 arrays, create a function that let's a user know (true/false) whether
  these two arrays contain any common items

  For Example:

    const array1 = ['a', 'b', 'c', 'x'];//const array2 = ['z', 'y', 'i'];
    should return false.

    -----------

    const array1 = ['a', 'b', 'c', 'x'];//const array2 = ['z', 'y', 'x'];
    should return true.

  2 parameters - arrays - no size limit
  return true or false
*/
/// Checks if two slices contain any common items using a nested loop approach.
/// This function directly translates the nested loop logic from JavaScript to Rust,
/// comparing each item in one array against all in the other.
///
/// # Arguments
/// * `arr1` - First slice of strings.
/// * `arr2` - Second slice of strings.
///
/// # Returns
/// * `bool` - Returns `true` if there is at least one common item, otherwise `false`.
fn contains_common_item(arr1: &[&str], arr2: &[&str]) -> bool {
    for &item1 in arr1 {
        for &item2 in arr2 {
            if item1 == item2 {
                return true;
            }
        }
    }
    false
}

/// Optimized version using a hash map to check for common items between two slices.
/// Implements a more optimized approach using Rust’s HashSet for constant-time look-up,
/// similar to the original JavaScript version that uses an object for caching.
///
/// # Arguments
/// * `arr1` - First slice of strings.
/// * `arr2` - Second slice of strings.
///
/// # Returns
/// * `bool` - Returns `true` if there is at least one common item, otherwise `false`.
fn contains_common_item2(arr1: &[&str], arr2: &[&str]) -> bool {
    use std::collections::HashSet;
    let mut map = HashSet::new();
    for &item in arr1 {
        map.insert(item);
    }

    for &item in arr2 {
        if map.contains(item) {
            return true;
        }
    }
    false
}

/// Simplified version leveraging iterators to find common items between two slices.
/// Uses Rust's iterator methods to find if any item in the first slice exists in
/// the second slice, providing a more idiomatic and concise approach in Rust.
///
/// # Arguments
/// * `arr1` - First slice of strings.
/// * `arr2` - Second slice of strings.
///
/// # Returns
/// * `bool` - Returns `true` if there is at least one common item, otherwise `false`.
fn contains_common_item3(arr1: &[&str], arr2: &[&str]) -> bool {
    arr1.iter().any(|item| arr2.contains(item))
}

fn main() {
    let array1 = ["a", "b", "c", "x"];
    let array2 = ["z", "y", "a"];

    println!(
        "Contains common item: {}",
        contains_common_item(&array1, &array2)
    );
    println!(
        "Contains common item 2: {}",
        contains_common_item2(&array1, &array2)
    );
    println!(
        "Contains common item 3: {}",
        contains_common_item3(&array1, &array2)
    );
}
