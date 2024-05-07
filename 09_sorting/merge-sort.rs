/// Merges two sorted slices into a single sorted vector.
///
/// Arguments:
/// * `left`: A sorted slice of i32.
/// * `right`: Another sorted slice of i32.
///
/// Returns:
/// A new vector containing all elements from `left` and `right`, sorted
/// in ascending order.
fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut left_index, mut right_index) = (0, 0);

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            result.push(left[left_index]);
            left_index += 1;
        } else {
            result.push(right[right_index]);
            right_index += 1;
        }
    }

    // Append the remaining elements of left or right
    if left_index < left.len() {
        result.extend_from_slice(&left[left_index..]);
    }
    if right_index < right.len() {
        result.extend_from_slice(&right[right_index..]);
    }

    println!("Left: {:?}, Right: {:?}", left, right);
    println!("Merged: {:?}", result);
    result
}

/// Recursively sorts a slice using the merge sort algorithm and returns a
/// new sorted vector.
///
/// Arguments:
/// * `arr`: The slice of i32 to be sorted.
///
/// Returns:
/// A new vector containing the sorted elements.
fn mergesort(arr: &[i32]) -> Vec<i32> {
    if arr.len() == 1 {
        return arr.to_vec();
    }
    let mid = arr.len() / 2;
    let left = &arr[..mid];
    let right = &arr[mid..];
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);

    merge(&mergesort(left), &mergesort(right))
}

fn main() {
    let arr = vec![99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
    let sorted = mergesort(&arr);
    println!("Sorted array: {:?}", sorted);
}

// TODO: add tests
