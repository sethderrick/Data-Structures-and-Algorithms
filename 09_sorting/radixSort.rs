/// Performs a radix sort on a slice of `i32`.
///
/// # Arguments
///
/// * `array` - A mutable slice of `i32` that will be sorted.
fn radix_sort(array: &mut [i32]) {
    let max_digit_count = most_digits(array);

    for k in 0..max_digit_count {
        let mut digit_buckets = vec![Vec::new(); 10]; // Create 10 buckets for digits 0-9

        for &num in array.iter() {
            let digit = get_digit(num, k);
            digit_buckets[digit as usize].push(num);
        }

        // Flatten digit_buckets and copy back to array
        let mut index = 0;
        for bucket in digit_buckets.iter() {
            for &num in bucket.iter() {
                array[index] = num;
                index += 1;
            }
        }
    }
}

/// Returns the k-th digit of a number counting from the right (0-indexed).
///
/// # Arguments
///
/// * `num` - The number from which to extract the digit.
/// * `place` - The digit place to extract.
fn get_digit(num: i32, place: usize) -> i32 {
    num.abs() / 10_i32.pow(place as u32) % 10
}

/// Counts the digits in a number.
///
/// # Arguments
///
/// * `num` - The number whose digits are to be counted.
fn digit_count(num: i32) -> usize {
    if num == 0 {
        1
    } else {
        (num.abs() as f64).log10().floor() as usize + 1
    }
}

/// Finds the maximum number of digits in any number within an array.
///
/// # Arguments
///
/// * `nums` - A slice of numbers to examine.
fn most_digits(nums: &[i32]) -> usize {
    nums.iter().map(|&num| digit_count(num)).max().unwrap_or(0)
}

fn main() {
    let mut numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
    radix_sort(&mut numbers);
    println!("Sorted numbers: {:?}", numbers);
}
