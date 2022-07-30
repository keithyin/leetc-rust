
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {

    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = (left + right) >> 1;
        if nums[mid] == target {
            break;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    let mut pos = (left + right) >> 1;
    if pos < nums.len() && nums[pos] < target {
        pos -= 1;
    }

    pos as i32
}