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
    strings.insert(2, 'alien');

    println!("{:?}", strings);
}