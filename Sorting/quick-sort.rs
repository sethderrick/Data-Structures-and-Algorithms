/// Swaps two elements in an array at given indices.
///
/// Arguments:
/// * `array`: The mutable slice of i32 integers to be modified.
/// * `first_index`: The index of the first element to swap.
/// * `second_index`: The index of the second element to swap.
fn swap(array: &mut [i32], first_index: usize, second_index: usize) {
    array.swap(first_index, second_index);
}

/// Partitions the array around a pivot element and returns the index of the pivot after partition.
///
/// Arguments:
/// * `array`: The mutable slice of i32 integers to be partitioned.
/// * `left`: The starting index of the slice to partition.
/// * `right`: The ending index of the slice to partition.
///
/// Returns:
/// The partition index where the pivot element resides after partitioning.
fn partition(array: &mut [i32], left: usize, right: usize) -> usize {
    let pivot_value = array[right];
    let mut partition_index = left;

    for i in left..right {
        if array[i] < pivot_value {
            swap(array, i, partition_index);
            partition_index += 1;
        }
    }

    swap(array, right, partition_index);
    partition_index
}

/// Sorts an array using the QuickSort algorithm.
///
/// Arguments:
/// * `array`: The mutable slice of i32 integers to be sorted.
/// * `left`: The starting index of the slice to sort.
/// * `right`: The ending index of the slice to sort.
///
/// Returns:
/// The sorted array (although the sorting is done in-place, so the original array is modified).
fn quicksort(array: &mut [i32], left: usize, right: usize) {
    if left < right {
        let partition_index = partition(array, left, right);

        if partition_index > 0 {
            // This check prevents underflow of usize
            quicksort(array, left, partition_index - 1);
        }
        quicksort(array, partition_index + 1, right);
    }
}

fn main() {
    let mut numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
    quicksort(&mut numbers, 0, numbers.len() - 1);
    println!("{:?}", numbers);
}

// TODO: add tests
