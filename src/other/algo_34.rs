

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let lower_bound = target as f32 - 0.1;
    let upper_bound = target as f32 + 0.1;
    let low_pos = find_insert_position(&nums, lower_bound);
    let high_pos = find_insert_position(&nums, upper_bound);
    return if low_pos == high_pos {
        vec![-1, -1]
    } else {
        vec![low_pos as i32, high_pos as i32 - 1]
    }
}

pub fn find_insert_position(nums: &Vec<i32>, target: f32) -> usize{
    let mut low = 0;
    let mut high = nums.len();
    while low < high {
        let mid = (low + high) / 2;
        if nums[mid] as f32 > target {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    return if low == 0 {
        0
    }else if low == nums.len() || nums[low] as f32 > target {
        low
    } else {
        low - 1
    }
}

#[cfg(test)]
mod test {
    use crate::other::algo_34::find_insert_position;

    #[test]
    fn test_insert_pos() {
        let nums = vec![1, 2, 3, 4];
        println!("{}", find_insert_position(&nums, 2.1));
    }
}