use std::collections::HashSet;
use std::str::FromStr;

pub fn subsets_with_dup_core(nums: &Vec<i32>, cur_pos: i32, left: i32,
                             result: &mut HashSet<String>, tracer: &mut Vec<String>) {
    if left == 0 {
        result.insert(tracer.join("_"));
        return;
    }

    if cur_pos as usize >= nums.len() {
        return;
    }

    tracer.push(nums[cur_pos as usize].to_string());
    subsets_with_dup_core(nums, cur_pos+1, left-1, result, tracer);
    tracer.pop();
    subsets_with_dup_core(nums, cur_pos+1, left, result, tracer);
}




pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = HashSet::new();
    nums.sort();
    for i in 1..=nums.len() {
        let mut tracer = vec![];
        subsets_with_dup_core(&nums, 0, i as i32, &mut result, &mut tracer);
    }
    let mut result: Vec<Vec<i32>> = result.iter().map(|x| x.split("_").map(|x| i32::from_str(x).unwrap()).collect()).collect();
    result.push(vec![]);
    result
}

#[cfg(test)]
mod test {
    use crate::lc::algo_90::subsets_with_dup;

    #[test]
    pub fn test() {
        println!("{:?}", subsets_with_dup(vec![1, 2, 3]));
    }
}


