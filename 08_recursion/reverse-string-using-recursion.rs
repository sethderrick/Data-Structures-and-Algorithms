/// Recursively reverses a string by printing the last character and then calling itself with the remainder of the string.
///
/// Arguments:
/// * `word`: A slice of the string to be reversed.
///
/// NOTE: Rust strings (&str) are UTF-8 encoded, so care must be taken when
/// slicing them not to split a character's bytes, which might lead to a runtime
/// panic. To avoid this, the chars() method is used to safely access the last
/// character, and len_utf8() provides its length in bytes, ensuring we slice
/// correctly.
/// ALSO: nstead of collecting characters and returning a new string, this
/// implementation directly prints each character. This avoids allocating memory
/// for a new string and is efficient in terms of space usage.
fn reverse(word: &str) {
    let size = word.len();
    if size == 0 {
        return;
    }
    // Get the last character of the string slice
    if let Some(last_char) = word.chars().last() {
        print!("{}", last_char);
        // Call reverse recursively on the string slice excluding the last character
        reverse(&word[..word.len() - last_char.len_utf8()]);
    }
}

fn main() {
    reverse("hello there");
    println!(); // to add a newline after the output
}

// TODO: add tests
