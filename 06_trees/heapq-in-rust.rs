/*
  In Rust, the equivalent functionality for Python's heapq is provided 
  by the BinaryHeap type in the std::collections module. 

  BinaryHeap Type: In Rust, BinaryHeap is a collection type that provides 
  the functionality of a priority queue. By default, BinaryHeap is a 
  max-heap, which means it allows easy retrieval of the largest value.

  Using Reverse for Min-Heap: To turn it into a min-heap (which behaves 
    like Python's heapq), we wrap the values in the Reverse tuple struct, 
    which reverses the default ordering.

  Heap Operations:
    • heapify: This is achieved by collecting the items of a vector into 
      a BinaryHeap.
    • heappush: Adding an element to the heap is done using the push method.
    • heappop: Removing the smallest element (or the largest in the case 
      of a standard BinaryHeap) is done with the pop method.
    • heappushpop: The push_pop method combines these two operations atomically.
    • nlargest and nsmallest: These operations are not directly supported 
      in the same way as Python’s heapq, but can be achieved by converting 
      the heap to a sorted vector and then taking the first or last elements.

  Efficiency: Rust's BinaryHeap operations such as push and pop operate in 
  O(log n) time, making it efficient for managing a dynamically changing dataset.
*/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// Demonstrates various operations using a min-heap in Rust using the 
/// BinaryHeap struct. BinaryHeap by default is a max-heap, so we use 
/// Reverse to make it behave as a min-heap.
fn main() {
    // Starting with a vector of elements.
    let mut x = vec![5, 2, 8, 1, 6, 7, 4, 9];

    // Convert the vector into a min-heap.
    let mut heap: BinaryHeap<Reverse<i32>> = x.into_iter().map(Reverse).collect();

    // Print the initial heapified vector (which will show in a heap-ordered 
    // format, not necessarily sorted).
    println!("{:?}", heap);

    // Adding an element to the heap.
    heap.push(Reverse(0));
    println!("{:?}", heap);

    // Removing the smallest element from the heap.
    println!("{}", heap.pop().unwrap().0);

    // Displaying the heap after popping the smallest element.
    println!("{:?}", heap);

    // Using heappushpop: This operation pushes a new element onto the heap 
    // and then pops and returns the smallest element from the heap in an 
    // atomic operation.
    println!("{}", heap.push_pop(Reverse(5)).0);
    println!("{:?}", heap);

    // To get the n largest elements, we need to convert to a max-heap 
    // temporarily since BinaryHeap::into_sorted_vec consumes the heap.
    let mut max_heap: BinaryHeap<_> = heap.iter().cloned().collect();
    let largest = max_heap.into_sorted_vec().into_iter().rev().take(4).collect::<Vec<_>>();
    println!("{:?}", largest);

    // Reset the heap for further use, displaying the 4 smallest elements 
    // using the already established min-heap behavior.
    heap = x.iter().map(|&num| Reverse(num)).collect();
    let smallest = heap.clone().into_sorted_vec().into_iter().take(4).collect::<Vec<_>>();
    println!("{:?}", smallest);
}

// TODO: add tests