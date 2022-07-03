
pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result = vec![];
    for first in 0..nums.len() {
        if first > 0 && nums[first] == nums[first-1] {
            continue;
        }

        for second in first+1..nums.len() {
            if (second - first) > 1 && nums[second] == nums[second-1] {
                continue;
            }


            let mut third = second + 1;
            let mut fourth = nums.len() - 1;
            while fourth > third {

                if (third-second) > 1 && nums[third] == nums[third-1] {
                    third += 1;
                    continue;
                }

                if (fourth + 1) < nums.len() && nums[fourth] == nums[fourth+1] {
                    fourth -= 1;
                    continue;
                }

                let cur = nums[first] as i64 + nums[second] as i64 + nums[third] as i64 + nums[fourth] as i64;
                let delta = cur - target as i64;
                if delta == 0 {
                    result.push(vec![nums[first], nums[second], nums[third], nums[fourth]]);
                    fourth -= 1;
                    third += 1;
                } else if delta > 0 {
                    fourth -= 1;
                } else {
                    third += 1;
                }
            }
        }
    }



    result
}

#[cfg(test)]
mod test {
    use crate::lc::algo_18::four_sum;

    #[test]
    pub fn test() {
        println!("{:?}", four_sum(vec![1000000000,1000000000,1000000000,1000000000], 0));
    }
}