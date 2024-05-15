use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut final_array: Vec<i32> = Vec::new();
    let mut obj_indices: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(&index) = obj_indices.get(&(target - num)) {
            final_array.push(index as i32);
            final_array.push(i as i32);
        } else {
            obj_indices.insert(*num, i);
        }
    }

    final_array
}

fn main() {
    println!("{:?}", two_sum(vec![3, 2, 4], 6));
}
