use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut map = HashMap::new();

    for pos in 0..nums.len() {
        let residual = target - nums[pos];
        if map.contains_key(&residual) {
            return vec![map[&residual], pos as i32];
        } else {
            map.insert(nums[pos], pos as i32);
        }
    }

    vec![]
}