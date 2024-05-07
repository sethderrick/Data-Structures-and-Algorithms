/// Sorts an array of integers using the bubble sort algorithm.
///
/// Arguments:
/// * `arr`: A mutable slice of i32 integers to be sorted.
fn bubblesort(arr: &mut [i32]) {
    let mut qw = 0;
    let len = arr.len();
    while qw < len {
        for i in 0..len - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
        qw += 1;
    }
}

fn main() {
    let mut arr = [5, 9, 1, 2, 7, 3, 8, 2];
    bubblesort(&mut arr);
    println!("{:?}", arr);
}

// TODO: add tests
