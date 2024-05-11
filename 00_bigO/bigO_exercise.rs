fn fun_challenge(input: &[i32]) -> i32 {
    let mut a = 10;
    a = 50 + 3;

    for _ in input.iter() {
        another_function();
        let _stranger = true;
        a += 1;
    }
    a
}

fn another_function() {
    // Assuming another_function does something simple
    println!("Function called");
}

fn main() {
    let input = vec![1, 2, 3, 4, 5]; // Example vector of integers
    let result = fun_challenge(&input);
    println!("Result: {}", result);
}
