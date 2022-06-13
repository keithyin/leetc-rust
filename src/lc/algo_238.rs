
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut left_cum_prod = vec![1; nums.len()];
    let mut right_cum_prod = vec![1; nums.len()];

    for i in 1..nums.len() {
        left_cum_prod[i] = left_cum_prod[i-1] * nums[i-1];
    }

    for i in (0..nums.len()-1).rev() {
        right_cum_prod[i] = right_cum_prod[i+1] * nums[i+1];
    }

    let mut res = vec![0; nums.len()];
    for i in 0..nums.len() {
        res[i] = right_cum_prod[i] * left_cum_prod[i];
    }

    res
}