fn boooo(n: usize) {
    // #5 Space complexity is O(1).
    for _ in 0..n {
        println!("booooo");
    }
}

fn array_of_hi_n_times(n: usize) -> Vec<&'static str> {
    // #6 Space complexity is O(n).
    let mut hi_array = Vec::new();
    for _ in 0..n {
        hi_array.push("hi");
    }
    hi_array
}

fn main() {
    boooo(6);

    let hi_array = array_of_hi_n_times(6);
    println!("{:?}", hi_array);
}
