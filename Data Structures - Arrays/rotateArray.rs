fn rotate(nums: &mut Vec<i32>, k: usize) {
    for _ in 0..k {
        if let Some(int_num) = nums.pop() {
            nums.insert(0, int_num);
        }
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    rotate(&mut nums, k);
    println!("{:?}", nums);
}
