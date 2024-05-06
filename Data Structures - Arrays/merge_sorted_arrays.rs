fn merge_sorted_arr<T: Ord + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    if a.is_empty() || b.is_empty() {
        return if a.is_empty() { b.to_vec() } else { a.to_vec() };
    }

    let mut my_list = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            my_list.push(a[i]);
            i += 1;
        } else {
            my_list.push(b[j]);
            j += 1;
        }
    }

    // Append remaining elements
    if i < a.len() {
        my_list.extend_from_slice(&a[i..]);
    }
    if j < b.len() {
        my_list.extend_from_slice(&b[j..]);
    }

    my_list
}

fn main() {
    let a = [1, 3, 4, 6, 20];
    let b = [2, 3, 4, 5, 6, 9, 11, 76];
    let result = merge_sorted_arr(&a, &b);
    println!("{:?}", result);
}
