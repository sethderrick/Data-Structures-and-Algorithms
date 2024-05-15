fn main() {
    let mut strings = vec!['a', 'b', 'c', 'd'];
    let mut numbers = vec![1, 2, 3, 4, 5];

    // push
    strings.push('e');

    // pop
    strings.pop();
    strings.pop();

    // unshift
    strings.insert(0, 'x');

    // splice
    // Commented out because it causes an error...Explaination follows below
    // strings.insert(2, 'alien');
    // NOTE: although this is the example given in the original JavaScript
    // the Rust equivalent is not as simple as the JavaScript one
    // Rust's character literals, enclosed in single quotes ('), can only
    // contain a single Unicode codepoint. However, in the original JavaScript,
    // the instructor inserted a string ('alien') into a vector of characters.
    // In Rust that causes an error. To fix this, you need to change the vector
    // type from a vector of characters (Vec<char>) to a vector of strings (Vec<String>),
    // and use double quotes (") for string literals. Like this:
    // let mut strings = vec![String::from("a"), String::from("b"), String::from("c"), String::from("d")];
    // strings.insert(2, "alien");

    println!("{:?}", strings);
}
