
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut begin = 0;
    let mut end = nums.len();

    let mut counter = 0;

    while begin < end {
        let mid = (begin + end) / 2;
        let mid_val = nums[mid];
        println!("begin:{}, end:{}, mid:{}", begin, end, mid);
        counter += 1;
        if counter > 10 {
            break;
        }
        if mid_val == target {
            return mid as i32;
        } else if mid_val < target {
            // 目标 > 中间值。判断当前的全局情况
            if mid_val >= nums[begin] {
                begin = mid+1;
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
    -1
}

#[cfg(test)]
mod test {
    use crate::lc::algo_33::search;

    #[test]
    pub fn test() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 3;
        search(nums, target);
    }
}