use std::cmp::{max, min};

pub fn max_product(nums: Vec<i32>) -> i32 {
    if nums.len() <= 0 {
        return 0;
    }
    let mut dp = vec![];
    dp.push((nums[0], nums[0]));

    for i in 1..nums.len() {
        let cur_max = max(max(nums[i], nums[i] * dp[i-1].0), nums[i] * dp[i-1].1);
        let cur_min = min(min(nums[i], nums[i] * dp[i-1].0), nums[i] * dp[i-1].1);
        dp.push((cur_max, cur_min));
    }
    dp.iter().map(|v| v.0).max().unwrap()
}