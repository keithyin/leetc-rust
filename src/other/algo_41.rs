
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums = nums.iter().map(|x| *x-1).collect();

    let mut idx = 0;
    while idx < nums.len() {
        if nums[idx] == idx as i32 || nums[idx] < 0 || nums[idx] >= nums.len() as i32 || nums[idx] == nums[nums[idx] as usize]{
            idx += 1;
            continue
        }

        let tmp = nums[idx] as usize;
        nums[idx] = nums[tmp];
        nums[tmp] = tmp as i32;

    }
    for (i, v) in nums.iter().enumerate() {
        if i as i32 != *v {
            return i as i32 + 1
        }
    }

    nums.len() as i32
}


#[cfg(test)]
mod test {
    use crate::other::algo_41::first_missing_positive;

    #[test]
    fn test_first_missing_positive() {
        println!("{}", first_missing_positive(vec![1,2,0]));
    }

}
