use std::collections::{HashMap, HashSet};

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    for v in nums.iter() {
        set.insert(*v);
    }

    let mut longest = 0;
    for v in nums.iter() {
        if set.contains(&(*v - 1)) {
            continue;
        }

        for i in 1..(nums.len() as i32 + 1) {
            if !set.contains(&(*v + i)) {
                longest = if longest > i {
                    longest
                } else {
                    i
                };
                break;
            }
        }
    }

    longest
}