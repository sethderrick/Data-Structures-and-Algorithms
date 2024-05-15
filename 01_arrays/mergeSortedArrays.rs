fn merge_sorted_arrays(array1: &[i32], array2: &[i32]) -> Vec<i32> {
    let mut merged_array = Vec::new();
    let mut i = 0;
    let mut j = 0;

    // Check if either array is empty
    if array1.is_empty() {
        return array2.to_vec();
    }
    if array2.is_empty() {
        return array1.to_vec();
    }

    while i < array1.len() || j < array2.len() {
        if j >= array2.len() || (i < array1.len() && array1[i] < array2[j]) {
            merged_array.push(array1[i]);
            i += 1;
        } else {
            merged_array.push(array2[j]);
            j += 1;
        }
    }

    merged_array
}

fn main() {
    let merged_array = merge_sorted_arrays(&[0, 3, 4, 31], &[3, 4, 6, 30]);

    println!("{:?}", merged_array);
}
