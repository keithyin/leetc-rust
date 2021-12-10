
pub fn find_kth_largest(mut nums: Vec<i32>, mut k: i32) -> i32 {
    k -= 1;
    let mut begin = 0;
    let mut end = nums.len() as i32;
    while end > begin {
        let (pos, value) = find_position(&mut nums, begin, end);
        if pos == k {
            return value;
        }else if pos < k {
            begin = pos + 1;
        } else {
            end = pos;
        }
    }
    -1
}

pub fn find_position(nums: &mut Vec<i32>, mut begin: i32, mut end: i32) -> (i32, i32){
    let value = nums[begin as usize];
    end -= 1;
    while end > begin {
        while end > begin && nums[end as usize] < value {
            end -= 1;
        }
        if end > begin {
            nums[begin as usize] = nums[end as usize];
            begin += 1
        }
        while end > begin && value < nums[begin as usize] {
            begin += 1;
        }

        if end > begin {
            nums[end as usize] = nums[begin as usize];
            end -= 1;
        }
    }
    nums[begin as usize] = value;
    return (begin, value);
}

#[cfg(test)]
mod test {
    use crate::sort_related::algo_215::find_kth_largest;

    #[test]
    fn test_find_kth_largest() {
        let nums = vec![3, 4, 2, 1, 5];
        println!("{}", find_kth_largest(nums, 2));
    }
}