/// Reverses a string using an iterative approach.
///
/// # Arguments
///
/// * `input` - A string slice that is to be reversed.
///
/// # Returns
///
/// A new `String` that is the reverse of the input.
///
/// # Examples
///
/// ```
/// let reversed = reverse_string("hola");
/// assert_eq!(reversed, "aloh");
/// ```
fn reverse_string(input: &str) -> String {
    let mut reversed_array = Vec::new(); // Using a Vec to store characters

    // Convert the string into a vector of characters
    let mut array_str: Vec<char> = input.chars().collect();

    // Closure to recursively add characters to reversed_array
    fn add_to_array(array: &mut Vec<char>, reversed_array: &mut Vec<char>) {
        if let Some(character) = array.pop() {
            reversed_array.push(character);
            add_to_array(array, reversed_array);
        }
    }

    // Start the recursive addition
    add_to_array(&mut array_str, &mut reversed_array);

    // Collect the characters into a String
    reversed_array.iter().collect()
}

/// Reverses a string using a recursive approach.
///
/// # Arguments
///
/// * `input` - A string slice that is to be reversed.
///
/// # Returns
///
/// A new `String` that is the reverse of the input.
///
/// # Examples
///
/// ```
/// let reversed = reverse_string_recursive("hola");
/// assert_eq!(reversed, "aloh");
/// ```
fn reverse_string_recursive(input: &str) -> String {
    if input.is_empty() {
        String::new()
    } else {
        let mut chars = input.chars();
        let first_char = chars.next().unwrap();
        let rest = chars.as_str();

        reverse_string_recursive(rest) + &first_char.to_string()
    }
}

fn main() {
    let input = "hola";
    println!(
        "Iterative reverse of '{}': {}",
        input,
        reverse_string(input)
    );
    println!(
        "Recursive reverse of '{}': {}",
        input,
        reverse_string_recursive(input)
    );
}
