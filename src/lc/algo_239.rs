
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut queue = vec![];
    let mut left = 0 as usize;
    let mut right = 0 as usize;
    queue.push(0);
    while right < nums.len() {
        while let Some(last) = queue.last() {
            if nums[*last] <= nums[right] {
                queue.remove(queue.len()-1);
            }else {
                break;
            }
        }
        queue.push(right);

        if (right - left) >= k as usize {
            left += 1;
        }

        while let Some(v) = queue.first() {
            if *v < left {
                queue.remove(0);
            } else {
                break;
            }
        }

        if (right - left) == (k - 1) as usize {
            result.push(nums[queue[0]]);
        }

        right += 1;

    }

    result
}