use std::collections::VecDeque;

fn main() {
    let mut strings = VecDeque::new();
    
    // Initialize with some strings for demonstration
    strings.push_back("a".to_string());
    strings.push_back("b".to_string());
    strings.push_back("c".to_string());
    strings.push_back("d".to_string());

    println!("{}", strings[2]);

    strings.push_back("e".to_string());

    strings.pop_back();
    strings.pop_back();

    strings.push_front("x".to_string());

    // Note: No direct method to insert in the middle, so converting to Vec temporarily
    let mut temp_vec: Vec<_> = strings.into_iter().collect();
    temp_vec.insert(2, "alien".to_string());
    strings = temp_vec.into_iter().collect();

    // Printing the strings
    for string in &strings {
        print!("{} ", string);
    }
    println!(); // for new line after the loop
}
