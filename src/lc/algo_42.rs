use std::cmp::min;

pub fn trap(height: Vec<i32>) -> i32 {
    let mut left_max_vec = vec![0; height.len()];
    let mut right_max_vec = vec![0; height.len()];

    let mut left_max = 0;
    for i in 0..height.len() {
        left_max_vec[i] = left_max;
        left_max = if left_max > height[i] {
            left_max
        } else {
            height[i]
        };
    }

    let mut right_max = 0;
    for i in (0..height.len()).rev() {
        right_max_vec[i] = right_max;
        right_max = if right_max > height[i] {
            right_max
        } else {
            height[i]
        };
    }

    let mut result = 0;
    for i in 0..height.len() {
        let cur_contain = min(left_max_vec[i], right_max_vec[i]) - height[i];
        if cur_contain > 0 {
            result += cur_contain;
        }
    }

    result

}