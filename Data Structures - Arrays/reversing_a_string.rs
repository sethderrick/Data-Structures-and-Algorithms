fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let input = "I am theja";
    let reversed = reverse(input);
    println!("{}", reversed);
}
