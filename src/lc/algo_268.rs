use std::collections::{HashSet};

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut has_n = false;
    let mut set = HashSet::new();
    for v in nums.iter() {
        set.insert(*v);
    }
    for i in 0..=nums.len(){
        if !set.contains(&(i as i32)) {
            return i as i32;
        }
    }

    0

}

