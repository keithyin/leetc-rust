

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = nums.len();

    while low < high {
        let mid = (low + high) / 2;
        if nums[mid] == target {
            return mid as i32;
        }else if  target > nums[mid] {
            if nums[mid] > nums[low] {
                high = mid;
            } else {
                low = mid + 1;
            }
        } else {  // nums[mid] < target
            if target <= nums[high-1] {
                low = mid + 1;
            } else {
                high = mid;
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