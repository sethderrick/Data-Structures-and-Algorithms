fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut arr_sums = vec![nums[0]];
    let mut int_max = nums[0];

    for i in 1..nums.len() {
        let int_sum = arr_sums[i - 1] + nums[i];

        if int_sum > nums[i] {
            arr_sums.push(int_sum);
            int_max = int_max.max(int_sum);
        } else {
            arr_sums.push(nums[i]);
            int_max = int_max.max(nums[i]);
        }
    }

    int_max
}

fn main() {
    println!("{}", max_sub_array(vec![-2, 1]));
}
