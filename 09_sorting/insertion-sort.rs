/// Sorts an array of integers using the insertion sort algorithm.
///
/// Arguments:
/// * `arr`: A mutable slice of i32 integers to be sorted.
fn insertionsort(arr: &mut [i32]) {
    let length = arr.len();
    let mut i = 1;
    while i < length {
        let x = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > x {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = x;
        i += 1;
    }
}

fn main() {
    let mut arr = [6, 5, 3, 1, 8, 7, 2, 4];
    insertionsort(&mut arr);
    println!("{:?}", arr);
}

// TODO: add tests
