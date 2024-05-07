/// Sorts an array using the bubble sort algorithm.
///
/// # Arguments
///
/// * `array` - A mutable slice of `i32` that will be sorted in place.
///
/// # Examples
///
/// ```
/// let mut numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
/// bubble_sort(&mut numbers);
/// assert_eq!(numbers, [0, 1, 2, 4, 5, 6, 44, 63, 87, 99, 283]);
/// ```
fn bubble_sort(array: &mut [i32]) {
    let length = array.len();
    for _ in 0..length {
        for j in 0..length - 1 {
            // Adjust to prevent out-of-bounds access
            if array[j] > array[j + 1] {
                // Swap the numbers using Rust's efficient tuple unpacking
                array.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
    bubble_sort(&mut numbers);
    println!("Sorted numbers: {:?}", numbers);
}
