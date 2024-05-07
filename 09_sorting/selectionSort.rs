/// Sorts an array using the selection sort algorithm.
///
/// # Arguments
///
/// * `array` - A mutable slice of `i32` that will be sorted in place.
fn selection_sort(array: &mut [i32]) {
    let length = array.len();

    for i in 0..length - 1 {
        // Assume the minimum is the first element
        let mut min_index = i;

        // Test against elements after i to find the smallest
        for j in i + 1..length {
            if array[j] < array[min_index] {
                // Found new minimum; remember its index
                min_index = j;
            }
        }

        // Swap if we found a new minimum
        if min_index != i {
            array.swap(i, min_index);
        }
    }
}

fn main() {
    let mut numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
    selection_sort(&mut numbers);
    println!("Sorted numbers: {:?}", numbers);
}
