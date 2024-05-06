/// Sorts an array of integers using the selection sort algorithm.
///
/// Arguments:
/// * `arr`: A mutable slice of i32 integers to be sorted.
fn selectionsort(arr: &mut [i32]) {
    let len = arr.len();
    let mut i = 0;
    while i < len {
        // Assume the first element of the unsorted segment is the minimum.
        let mut min_index = i;
        for j in (i + 1)..len {
            // Update the minimum element if a smaller element is found.
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        // Swap the found minimum element with the first element of the
        // unsorted segment.
        arr.swap(i, min_index);
        i += 1;
    }
}

fn main() {
    let mut arr = [8, 6, 5, 0, 4, 3, 2];
    selectionsort(&mut arr);
    println!("{:?}", arr);
}
