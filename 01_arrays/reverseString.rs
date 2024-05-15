fn reverse(str: &str) -> String {
    if str.is_empty() || str.len() < 2 {
        return str.to_string();
    }

    let mut backwards = Vec::new();
    let total_items = str.len() - 1;
    for i in (0..=total_items).rev() {
        backwards.push(str.chars().nth(i).unwrap());
    }
    backwards.into_iter().collect()
}

fn reverse2(str: &str) -> String {
    str.chars().rev().collect()
}

fn main() {
    let reverse3 = |str: &str| str.chars().rev().collect::<String>();

    let result1 = reverse("Timbits Hi");
    let result2 = reverse2("Timbits Hi");
    let result3 = reverse3("Timbits Hi");

    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    println!("Result 3: {}", result3);
}
