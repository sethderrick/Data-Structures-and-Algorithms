fn move_zeroes(nums: Vec<i32>) -> Vec<i32> {
    let mut final_array: Vec<i32> = Vec::new();

    for i in (0..nums.len()).rev() {
        if nums[i] == 0 {
            final_array.push(0);
        } else {
            final_array.insert(0, nums[i]);
        }
    }

    final_array
}

fn main() {
    println!("{:?}", move_zeroes(vec![0, 1, 0, 3, 12]));
}
