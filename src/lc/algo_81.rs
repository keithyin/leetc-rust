pub fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut begin = 0;
    let mut end = nums.len();

    while begin < end {
        let mid = (begin + end) / 2;
        let mid_val = nums[mid];
        if mid_val == target {
            return true;
        }
        if mid_val == nums[begin] && mid_val == nums[end-1] {
            continue;
        }

        if target > mid_val {
            // 目标 > 中间值。判断当前的全局情况
            if mid_val >= nums[begin] { // 左边还是右边
                begin = mid + 1;
            } else if mid_val < nums[begin] {
                if target >= nums[begin] {
                    end = mid;
                } else {
                    begin = mid + 1;
                }
            }
        } else {
            // 目标小于中间值
            if mid_val >= nums[begin] {
                if target >= nums[begin] {
                    end = mid;
                } else {
                    begin = mid + 1;
                }
            } else {
                end = mid;
            }
        }
    }
    false
}
