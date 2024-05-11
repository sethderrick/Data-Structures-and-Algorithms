fn log_all_pairs_of_array(array: &[&str]) {
    for i in array.iter() {
        for j in array.iter() {
            println!("{}, {}", i, j);
        }
    }
}

fn main() {
    let boxes = ["a", "b", "c", "d", "e"];
    log_all_pairs_of_array(&boxes);
}
