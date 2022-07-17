use std::cmp::max;
use std::collections::HashMap;

pub fn rob_core(nums: &Vec<i32>, cur_pos: usize, mut first_stolen: bool, memo: &mut HashMap<(usize, bool), i32>) -> i32 {
    if cur_pos >= nums.len() {
        return 0;
    }
    if cur_pos == (nums.len()-1) && first_stolen {
        return 0;
    }

    if let Some(res) = memo.get(&(cur_pos, first_stolen)) {
        return *res;
    }

    let steal_cur = nums[cur_pos] + rob_core(nums, cur_pos+2, if cur_pos == 0 {true} else {first_stolen}, memo);
    let not_steal_cur = rob_core(nums, cur_pos+1, if cur_pos == 0 {false} else {first_stolen}, memo);
    let cur_res = max(steal_cur, not_steal_cur);
    memo.insert((cur_pos, first_stolen), cur_res);
    cur_res

}

pub fn rob(nums: Vec<i32>) -> i32 {
    rob_core(&nums, 0, false, &mut HashMap::new())
}


