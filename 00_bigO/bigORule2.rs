fn print_first_item_then_first_half_then_say_hi_100_times<T: std::fmt::Debug>(items: &[T]) {
    if let Some(first) = items.first() {
        println!("{:?}", first);
    }

    let middle_index = items.len() / 2;

    for item in &items[0..middle_index] {
        println!("{:?}", item);
    }

    for _ in 0..100 {
        println!("hi");
    }
}

fn main() {
    let items = vec![1, 2, 3, 4, 5, 6]; // Example vector of integers
    print_first_item_then_first_half_then_say_hi_100_times(&items);
}
