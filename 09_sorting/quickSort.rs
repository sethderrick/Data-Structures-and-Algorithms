/// Sorts an array using the quicksort algorithm.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the array to be sorted.
/// * `low` - The starting index of the portion of the array to be sorted.
/// * `high` - The ending index of the portion of the array to be sorted.
fn quick_sort(arr: &mut [i32], low: isize, high: isize) {
    if low < high {
        let pi = partition(arr, low, high);
        quick_sort(arr, low, pi - 1);
        quick_sort(arr, pi + 1, high);
    }
}

/// Partitions the array around the pivot element.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the array to be partitioned.
/// * `low` - The starting index of the portion of the array to be partitioned.
/// * `high` - The ending index of the portion of the array to be partitioned.
///
/// # Returns
///
/// The index of the pivot element after partitioning.
fn partition(arr: &mut [i32], low: isize, high: isize) -> isize {
    let pivot = arr[high as usize];
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize] < pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, high as usize);
    i + 1
}

fn main() {
    let mut numbers = vec![34, 7, 23, 32, 5, 62];
    let len = numbers.len(); // Separate the calculation of the length
    quick_sort(&mut numbers, 0, len as isize - 1);
    println!("Sorted array: {:?}", numbers);
}
