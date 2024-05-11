fn another_fun_challenge(input: usize) {
    let a = 5; // O(1)
    let b = 10; // O(1)
    let c = 50; // O(1)

    for i in 0..input {
        // O(n)
        let x = i + 1; // O(n)
        let y = i + 2; // O(n)
        let z = i + 3; // O(n)
    }

    for j in 0..input {
        // O(n)
        let p = j * 2; // O(n)
        let q = j * 2; // O(n)
    }

    let who_am_i = "I don't know"; // O(1)
}

fn main() {
    another_fun_challenge(5);
}
