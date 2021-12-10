fn catch_rain_brute_force(heights: &Vec<i32>) -> i32 {
    if heights.is_empty() {
        return 0;
    }
    let mut left_max = vec![];
    left_max.resize(heights.len(), 0);
    let mut right_max = vec![];
    right_max.resize(heights.len(), 0);
    let mut cursor = (heights.len() - 1) as i32;
    let mut left = heights[cursor as usize];
    while cursor >= 0 {
        let cur_val = heights[cursor as usize];
        left = if cur_val > left {
            cur_val
        } else {
            left
        };
        left_max[cursor as usize] = left;
        cursor -= 1
    }

    cursor = 0;
    let mut right = heights[0];
    while cursor < (heights.len() as i32) {
        let cur_val = heights[cursor as usize];
        right = if right > cur_val {
            right
        } else {
            cur_val
        };
        right_max[cursor as usize] = right;
        cursor += 1;
    }

    let mut capacity = 0;
    cursor = 0;
    while cursor < (heights.len() as i32) {
        let bottleneck = if right_max[cursor as usize] > left_max[cursor as usize] {
            left_max[cursor as usize]
        } else {
            right_max[cursor as usize]
        };
        capacity += (bottleneck - heights[cursor as usize]);
        cursor += 1;
    }

    return capacity;
}

#[cfg(test)]
mod test {
    use crate::stack::algo_42::catch_rain_brute_force;

    #[test]
    fn test_catch_rain() {
        let heights = vec![1, 2, 3, 2, 3];
        assert_eq!(catch_rain_brute_force(&heights), 1);
    }
}