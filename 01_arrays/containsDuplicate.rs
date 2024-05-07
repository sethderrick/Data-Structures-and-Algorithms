fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut obj_nums = std::collections::HashSet::new();

    for int_num in nums {
        if obj_nums.contains(&int_num) {
            return true;
        }

        obj_nums.insert(int_num);
    }

    false
}

fn main() {
    println!("{}", contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
}
