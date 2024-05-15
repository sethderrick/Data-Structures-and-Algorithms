// NOTE: This will not compile because the regex crate is not declared in the code
// This is for viewing purposes only. To use, make sure to add regex to your dependencies
// in your Cargo.toml file and reference the crate in your code.
fn longest_word(sen: &str) -> &str {
    let arr_words: Vec<&str> = sen.split_whitespace().collect();
    let regular_expression = regex::Regex::new(r"^[a-zA-Z0-9ñáéíóúÁÉÍÓÚÑ ,.'-]+$").unwrap();

    let mut str_longer = "";

    for string_sen in arr_words {
        if regular_expression.is_match(string_sen) && string_sen.len() > str_longer.len() {
            str_longer = string_sen;
        }
    }

    str_longer
}

fn main() {
    let input = String::from("Hello world! This is a test.");
    let longest = longest_word(&input);
    println!("{}", longest);
}
