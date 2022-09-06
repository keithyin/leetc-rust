
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut cursor = 0;
    let mut counter = 1;
    for i in 1..nums.len() {
        let cur = nums[i];
        if cur == nums[cursor] {
            if counter >= 2 {
                continue
            }
            counter += 1;
            cursor += 1;
            nums[cursor] = cur;
        } else {
            counter = 1;
            cursor += 1;
            nums[cursor] = cur;
        }
    }
    cursor as i32 + 1
}