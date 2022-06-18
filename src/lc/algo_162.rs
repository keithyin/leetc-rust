



pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut begin = 0;
    let mut end = nums.len() as i32 - 1;

    while begin <= end {


        let mid = (begin + end) / 2;
        if (mid - 1) >= 0 && (mid + 1) < nums.len() as i32 {

            if nums[mid as usize] > nums[mid as usize - 1] && nums[mid as usize] > nums[mid as usize + 1] {
                return mid;
            }

            if nums[mid as usize] > nums[mid as usize - 1] && nums[mid as usize] < nums[mid as usize + 1]{
                begin = mid + 1;
            } else if nums[mid as usize] < nums[mid as usize - 1] && nums[mid as usize] > nums[mid as usize + 1] {
                end = mid - 1;
            } else {
                begin = mid + 1;
            }

        } else if (mid - 1) >= 0{
            return if nums[mid as usize] > nums[mid as usize - 1] {
                mid
            } else {
                mid - 1
            };
        } else if (mid + 1) < nums.len() as i32{
            return if nums[mid as usize] > nums[mid as usize + 1] {
                mid
            } else {
                mid + 1
            };
        } else {
            return 0;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use crate::lc::algo_162::find_peak_element;

    #[test]
    fn test_find_peak_element() {
        println!("{}", find_peak_element(vec![1, 2]));
    }
}