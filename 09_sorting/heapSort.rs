/// Sorts an array using the heap sort algorithm.
/// Made use of array.swap() as it simplifies the swapping logic compared to
/// the manual swapping in the original JavaScript in the aneagoie repo
///
/// # Arguments
///
/// * `array` - A mutable slice of `i32` that will be sorted in place.
fn heap_sort(array: &mut [i32]) {
    let mut heap_size = array.len();

    // Build a max heap from the array
    build_heap(array);

    for i in (1..heap_size).rev() {
        // Swap the root (maximum value) of the heap with the last element of the heap
        array.swap(i, 0);

        // Reduce the size of the heap so that the previous max value will stay in its proper place
        heap_size -= 1;

        // Heapify the root element again to get the highest element at the root again
        max_heapify(array, heap_size, 0);
    }
}

/// Builds a max heap from an unsorted array.
///
/// # Arguments
///
/// * `array` - A mutable slice of `i32` representing the heap.
fn build_heap(array: &mut [i32]) {
    let heap_size = array.len();
    let start = (heap_size / 2).saturating_sub(1); // Start from the last parent node

    for i in (0..=start).rev() {
        max_heapify(array, heap_size, i);
    }
}

/// Ensures the sub-tree rooted at `i` is a max heap.
///
/// # Arguments
///
/// * `array` - A mutable slice of `i32` representing the heap.
/// * `heap_size` - The number of elements in the heap.
/// * `i` - The index of the root element of the sub-tree.
fn max_heapify(array: &mut [i32], heap_size: usize, i: usize) {
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    let mut largest = i;

    // If the left child is larger than the current largest, update largest
    if left < heap_size && array[left] > array[largest] {
        largest = left;
    }

    // If the right child is larger than the current largest, update largest
    if right < heap_size && array[right] > array[largest] {
        largest = right;
    }

    // If the largest is not the current node, swap it with the largest and heapify again
    if largest != i {
        array.swap(i, largest);
        max_heapify(array, heap_size, largest);
    }
}

fn main() {
    let mut numbers = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
    heap_sort(&mut numbers);
    println!("Sorted numbers: {:?}", numbers);
}
