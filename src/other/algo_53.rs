
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_val = i32::MIN;
    let mut cursor = i32::MIN;
    for v in nums.iter() {
        if cursor < 0 {
            cursor = *v;
        } else {
            cursor += *v;
        }
        max_val = if cursor > max_val {
            cursor
        } else {
            max_val
        };
    }
    max_val
}

#[cfg(test)]
mod test {
    use crate::other::algo_53::max_sub_array;

    #[test]
    fn test_max_sub_array() {
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
        println!("{}", max_sub_array(nums));
    }
}