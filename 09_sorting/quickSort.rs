/// Performs an in-place quicksort on a slice of `i32`.
///
/// # Arguments
///
/// * `array` - A mutable slice of `i32` that will be sorted.
/// * `left` - The starting index for the sorting process.
/// * `right` - The ending index for the sorting process.
fn quick_sort(array: &mut [i32], left: usize, right: usize) {
    if left < right {
        let partition_index = partition(array, left, right);
        if partition_index > 0 {
            quick_sort(array, left, partition_index - 1);
        }
        quick_sort(array, partition_index + 1, right);
    }
}

/// Partitions the array around a pivot element and returns the index of the pivot after partition.
///
/// # Arguments
///
/// * `array` - A mutable slice of `i32`.
/// * `left` - The starting index for the partition.
/// * `right` - The ending index for the partition.
///
/// # Returns
///
/// The partition index where the pivot element resides after partitioning.
fn partition(array: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = array[right];
    let mut i = left;

    for j in left..right {
        if array[j] < pivot {
            array.swap(i, j);
            i += 1;
        }
    }

    array.swap(i, right);
    i
}

fn main() {
    let mut numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
    quick_sort(&mut numbers, 0, numbers.len() - 1);
    println!("Sorted numbers: {:?}", numbers);
}
