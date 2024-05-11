fn print_all_numbers_then_all_pair_sums(numbers: &[i32]) {
    // Printing all the numbers
    println!("these are the numbers:");
    for &number in numbers {
        println!("{}", number);
    }

    // Printing all possible sums of pairs of numbers
    println!("and these are their sums:");
    for &first_number in numbers {
        for &second_number in numbers {
            println!("{}", first_number + second_number);
        }
    }
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    print_all_numbers_then_all_pair_sums(&numbers); // O(n^2) due to nested loops
}
