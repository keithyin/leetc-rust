
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut counter = 0;
    for i in 0..nums.len() {
        nums[i-counter] = nums[i];
        if val == nums[i] {
            counter+=1;
        }
    }
    (nums.len() - counter) as i32
}

#[cfg(test)]
mod test {
    use crate::lc::algo_27::remove_element;

    #[test]
    pub fn test() {
        let mut nums = vec![1, 2, 3, 4, 2, 5, 6];
        let val = 2;
        println!("{}", remove_element(&mut nums, val));
        println!("{:?}", nums);
    }
}