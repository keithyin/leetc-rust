use std::cmp::max;
use std::collections::HashMap;

fn rob_core_198(nums: &Vec<i32>, cur_pos: usize, tracer: &mut HashMap<usize, (i32, i32)>) -> i32 {
    if cur_pos >= nums.len() {
        return 0;
    }

    if let Some(pair) = tracer.get(&cur_pos) {
        return max(pair.0, pair.1);
    }

    let rob_this_value  = nums[cur_pos] + rob_core_198(nums, cur_pos+2, tracer);
    let not_rob_this_value = rob_core_198(nums, cur_pos+1, tracer);
    tracer.insert(cur_pos, (rob_this_value, not_rob_this_value));

    max(rob_this_value, not_rob_this_value)
}

pub fn rob_198(nums: Vec<i32>) -> i32 {
    let mut tracer = HashMap::new();
    rob_core_198(&nums, 0, &mut tracer)
}