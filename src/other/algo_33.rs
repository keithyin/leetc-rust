

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = nums.len();

    while low < high {
        println!("low:{}, high:{}", low, high);
        let mid = (low + high) / 2;
        if nums[mid] == target {
            return mid as i32;
        }else if nums[high-1] >= nums[low] {
            if target > nums[mid] {
                low = mid + 1;
            } else {
                high = mid
            }
        } else if nums[high-1] < nums[low] {  // nums[mid] < target
            if nums[mid] < nums[low] {
                if target > nums[mid] && target < nums[low] {
                    low = mid+1;
                } else {
                    high = mid;
                }
            } else {
                if target > nums[mid] || target < nums[low] {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }

        }
    }
    return -1;
}

#[cfg(test)]
mod test {
    use crate::other::algo_33::search;

    #[test]
    fn test_search() {
        let nums = vec![4,5,6,7,0,1,2];
        assert_eq!(search(nums, 0), 4);
        let nums = vec![4,5,6,7,0,1,2];
        assert_eq!(search(nums, 3), -1);

        let nums = vec![7,8,1,2,3,4,5,6];
        println!("{}", search(nums, 2));
    }
}