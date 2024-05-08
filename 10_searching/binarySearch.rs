/*
  NOTE: The original repo did not include the implementation. It only presented the shell of the
        binary_search() function. I've written this including an implementation.
*/

/// Performs a binary search on a slice of `i32`.
///
/// # Arguments
///
/// * `array` - A sorted slice of `i32`.
/// * `num` - The number to search for.
///
/// # Returns
///
/// An `Option<usize>` that is `Some(index)` if the number is found, where `index` is the
/// position of the number in the array.
/// Returns `None` if the number is not found.
fn binary_search(array: &[i32], num: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = array.len() as isize - 1;

    while start <= end {
        let mid = start + (end - start) / 2;
        if array[mid as usize] == num {
            return Some(mid as usize);
        } else if array[mid as usize] < num {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    None
}

fn main() {
    let numbers: [i32; 17] = [
        1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
    ];
    let num_to_search = 37;
    match binary_search(&numbers, num_to_search) {
        Some(index) => println!("Number {} found at index {}", num_to_search, index),
        None => println!("Number {} not found in the array", num_to_search),
    }
}
