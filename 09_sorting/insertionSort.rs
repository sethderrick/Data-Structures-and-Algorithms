/// Sorts an array using the insertion sort algorithm.
///
/// # Arguments
///
/// * `array` - A mutable slice of `i32` that will be sorted in place.
///
/// # Examples
///
/// ```
/// let mut numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
/// insertion_sort(&mut numbers);
/// assert_eq!(numbers, [0, 1, 2, 4, 5, 6, 44, 63, 87, 99, 283]);
/// ```
fn insertion_sort(array: &mut [i32]) {
    let length = array.len();

    for i in 1..length {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            // Swap elements if they are in the wrong order
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    let mut numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
    insertion_sort(&mut numbers);
    println!("Sorted numbers: {:?}", numbers);
}
