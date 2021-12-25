pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut max_position = 0;
    let mut res = false;
    for (i, v) in nums.iter().enumerate() {
        if i > max_position {
            break;
        }
        let current_max_pos = *v as usize + i;
        max_position = if current_max_pos > max_position {
            current_max_pos
        } else {
            max_position
        };

        if max_position >= (nums.len() - 1) {
            res = true;
            break;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use crate::other::algo_55::can_jump;

    #[test]
    fn test_can_jump() {
        let nums = vec![2,3,1,1,4];
        println!("{}", can_jump(nums));
    }
}