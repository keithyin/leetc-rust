use std::cmp::max;

pub fn increasing_triplet(nums: Vec<i32>) -> bool {

    let mut left_min = nums.clone();
    let mut right_max = nums.clone();

    let mut min_val = i32::MAX;

    for i in 0..nums.len() {
        min_val = if nums[i] >= min_val {
            min_val
        } else {
            nums[i]
        };
        left_min[i] = min_val;
    }


    let mut max_val = i32::MIN;
    for i in (0..nums.len()).rev() {
        max_val = if nums[i] < max_val {
            max_val
        } else {
            nums[i]
        };
        right_max[i] = max_val;
    }

    for i in 0..nums.len() {
        if nums[i] > left_min[i] && nums[i] < right_max[i] {
            return true;
        }
    }


    false
}


#[cfg(test)]
mod test {
    use crate::lc::algo_334::increasing_triplet;

    #[test]
    fn test_increasing_triplet() {
        let nums = vec![5,4,3,2,1];
        println!("{}", increasing_triplet(nums));
    }
}