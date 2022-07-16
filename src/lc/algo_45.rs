
pub fn jump(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums.len()];
    for i in 1..nums.len() {
        let mut min_jump_time = i32::MAX;
        for j in (0..i).rev() {
            if (nums[j] as usize + j) >= i {
                min_jump_time = if (dp[j] + 1) <= min_jump_time {
                    dp[j] + 1
                } else {
                    min_jump_time
                };
            }
        }
        dp[i] = min_jump_time;
    }
    dp[nums.len()-1]
}