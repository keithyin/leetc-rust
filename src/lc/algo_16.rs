
pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort();
    let mut cur_min_delta = i32::MAX;
    let mut closet_sum = 0;

    for first in 0..nums.len() {
        if first > 0 && nums[first] == nums[first-1] {
            continue;
        }
        let mut third = nums.len()-1;
        let mut second = first + 1;

        while third > second{
            if (third + 1) < nums.len() && nums[third] == nums[third+1] {
                third -= 1;
                continue
            };
            if (second-first) > 1 && nums[second] == nums[second-1] {
                second += 1;
                continue;
            }

            let cur_val = nums[first] + nums[second] + nums[third];
            let cur_delta = cur_val - target;
            if cur_delta.abs() < cur_min_delta {
                cur_min_delta = cur_delta.abs();
                closet_sum = cur_val;
            }
            if cur_delta == 0 {
                break;
            }else if cur_delta > 0 {
                third -= 1;
            } else {
                second += 1;
            }
        }
    }
    closet_sum
}

#[cfg(test)]
mod test {
    use crate::lc::algo_16::three_sum_closest;

    #[test]
    pub fn test() {
        println!("{}", three_sum_closest(vec![1, 1, 1, 1], 0));
    }
}

