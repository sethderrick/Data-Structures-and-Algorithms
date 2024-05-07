/*

Google Question
Given an array = [2,5,1,2,3,5,1,2,4]:
It should return 2

Given an array = [2,1,1,2,3,5,1,2,4]:
It should return 1

Given an array = [2,3,4,5]:
It should return undefined

Bonus... What if we had this:
 [2,5,5,2,3,5,1,2,4]
 return 5 because the pairs are before 2,2

*/

fn first_recurring_character(input: &[i32]) -> Option<i32> {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if input[i] == input[j] {
                return Some(input[i]);
            }
        }
    }
    None
}

fn first_recurring_character2(input: &[i32]) -> Option<i32> {
    let mut map = std::collections::HashMap::new();
    for (i, &num) in input.iter().enumerate() {
        if let Some(&index) = map.get(&num) {
            return Some(input[index]);
        } else {
            map.insert(num, i);
        }
    }
    None
}

fn main() {
    let input = [1, 5, 5, 1, 3, 4, 6];
    if let Some(result) = first_recurring_character2(&input) {
        println!("First recurring character: {}", result);
    } else {
        println!("No recurring character found");
    }
}
