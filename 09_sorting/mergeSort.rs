/// Performs merge sort on a slice of `i32`.
///
/// # Arguments
///
/// * `array` - A slice of `i32` that will be sorted.
///
/// # Returns
///
/// A `Vec<i32>` containing the sorted elements.
fn merge_sort(array: &[i32]) -> Vec<i32> {
    if array.len() == 1 {
        return array.to_vec();
    }

    // Split Array into right and left at the middle index
    let middle = array.len() / 2;
    let left = merge_sort(&array[..middle]);
    let right = merge_sort(&array[middle..]);

    merge(&left, &right)
}

/// Merges two sorted lists into one sorted list.
///
/// # Arguments
///
/// * `left` - A sorted slice of `i32`.
/// * `right` - A sorted slice of `i32`.
///
/// # Returns
///
/// A `Vec<i32>` containing the merged and sorted elements.
fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_index = 0;
    let mut right_index = 0;

    // Merge the two sorted lists
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            result.push(left[left_index]);
            left_index += 1;
        } else {
            result.push(right[right_index]);
            right_index += 1;
        }
    }

    // Append the remaining elements
    result.extend_from_slice(&left[left_index..]);
    result.extend_from_slice(&right[right_index..]);

    result
}

fn main() {
    let numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
    let sorted_numbers = merge_sort(&numbers);
    println!("Sorted numbers: {:?}", sorted_numbers);
}
