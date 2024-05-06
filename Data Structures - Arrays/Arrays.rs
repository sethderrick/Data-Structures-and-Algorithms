fn main() {
    /*
        Using vec instead of array because arrays in Rust are not dynamic.
        Arrays in Rust are fixed size. Also note...
        Calculating storage used by 'strings' is not as straightforward 
        in Rust as it might be in other languages. There are several factors 
        to consider:
            1. The `Vec` itself: A `Vec` in Rust has three components: a pointer 
            to the data, the length of the vector, and the capacity of the vector. 
            On a 64-bit system, each of these is 8 bytes, so the `Vec` itself 
            takes up 24 bytes.

            2. The string slices: Each string slice is a reference to a string 
            literal, which consists of a pointer to the start of the string 
            and the length of the string. On a 64-bit system, each of these is 
            8 bytes, so each string slice is 16 bytes. Since there are four string 
            slices, they take up 64 bytes in total.

            3. The string literals: The string literals "a", "b", "c", and "d" 
            are each one byte long. However, string literals in Rust are 
            null-terminated, so they each take up 2 bytes in memory. Since there 
            are four string literals, they take up 8 bytes in total.

        So, the total storage used by "string" is 24 bytes (for the `Vec`) + 64 bytes 
        (for the string slices) + 8 bytes (for the string literals) = 96 bytes.
     */
    
    let mut strings = vec!["a", "b", "c", "d"];

    // Print the third element (0-indexed)
    println!("{}", strings[2]);

    // Append an element at the end
    strings.push("e");

    // Remove the last element
    strings.pop();
    strings.pop();

    // Insert an element at the start
    // This operation is O(n) because it may shift all elements in the buffer
    strings.insert(0, "x");

    // Insert an element at the second position
    // This is also O(n) for the same reason as above
    strings.insert(2, "alien");

    // Print the array
    println!("{:?}", strings);

    
    // Additional Rust Vec operations (commented for clarity)

    // clear() - Removes all the elements from the Vec, which makes it empty
    // strings.clear();

    // clone() - Returns a copy of the Vec (similar to Python's copy())
    // let cloned_strings = strings.clone();

    // contains() - Returns true if the Vec contains the specified value (similar to Python's count() but for existence check)
    // let contains_a = strings.contains(&"a");

    // extend() - Adds elements of a slice or another Vec (similar to Python's extend())
    // strings.extend(["new1", "new2"].iter());

    // find() - Returns the index of the first element with the specified value (similar to Python's index())
    // let index_of_a = strings.iter().position(|&r| r == "a");

    // remove() - Removes the element at a specified position (not directly by value)
    // let removed_item = strings.remove(0);

    // reverse() - Reverses the order of the elements in the Vec in place
    // strings.reverse();

    // sort() - Sorts the list in place (only works if the elements implement the Ord trait)
    // strings.sort();
}

// Link to Rust's Vec documentation for in-depth information
// https://doc.rust-lang.org/std/vec/struct.Vec.html