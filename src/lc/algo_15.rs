use std::collections::{HashMap, HashSet};

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut results = HashSet::new();
    for i in 0..nums.len() {
        let anchor = - nums[i];
        let mut residual_map = HashSet::new();
        for j in i+1..nums.len() {
            let cur = nums[j];
            if residual_map.contains(&cur) {
                results.insert((-anchor, anchor - cur, cur));
            }
            residual_map.insert(anchor - cur);
        }
    }

    results.iter().map(|item| vec![item.0, item.1, item.2]).collect()
}

#[cfg(test)]
mod test {
    use crate::lc::algo_15::three_sum;

    #[test]
    pub fn test() {
        println!("{:?}", three_sum(vec![-1,0,1,2,-1,-4]));
    }
}