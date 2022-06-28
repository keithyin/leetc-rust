
pub fn majority_element(nums: Vec<i32>) -> i32 {

    let mut cur_v = nums[0];
    let mut count = 1;
    for i in 1..nums.len() {
        if cur_v == nums[i] {
            count += 1;
        } else {
            count -= 1;
            if count < 0 {
                count = 1;
                cur_v = nums[i]
            }
        }
    }

    cur_v

}