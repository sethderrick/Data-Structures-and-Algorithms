/// Performs a counting sort on a slice of `i32`.
///
/// # Arguments
///
/// * `arr` - A slice of `i32` to be sorted.
/// * `max` - The maximum value in the array.
/// * `min` - The minimum value in the array.
///
/// # Returns
///
/// A new Vec<i32> containing the sorted elements.
///
/// # Examples
///
/// ```
/// let numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
/// let sorted_numbers = counting_sort(&numbers, *numbers.iter().max().unwrap(), *numbers.iter().min().unwrap());
/// assert_eq!(sorted_numbers, vec![0, 1, 2, 4, 5, 6, 44, 63, 87, 99, 283]);
/// ```
fn counting_sort(arr: &[i32], max: i32, min: i32) -> Vec<i32> {
    let mut count = vec![0; (max - min + 1) as usize];

    // Increment count array based on the frequency of each element in the input array
    for &value in arr {
        count[(value - min) as usize] += 1;
    }

    let mut sorted_arr = Vec::new();

    // Build the sorted array by repeating elements according to their count
    for i in 0..count.len() {
        let value = i as i32 + min;
        for _ in 0..count[i] {
            sorted_arr.push(value);
        }
    }

    sorted_arr
}

fn main() {
    let numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
    let answer = counting_sort(
        &numbers,
        *numbers.iter().max().unwrap(),
        *numbers.iter().min().unwrap(),
    );
    println!("Sorted numbers: {:?}", answer);
}
