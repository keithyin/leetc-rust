pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut hit = vec![0; nums.len()];
    let mut tracer = vec![];
    let mut result: Vec<Vec<i32>> = vec![];
    core(&nums, &mut hit, &mut tracer, &mut result);
    result
}

pub fn core(nums: &Vec<i32>, hit: &mut Vec<i32>, tracer: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if hit.iter().sum::<i32>() == hit.len() as i32 {
        result.push(tracer.clone());
        return;
    }
    for i in 0..hit.len(){
        if hit[i] == 0 {
            hit[i] = 1;
            tracer.push(nums[i]);
            core(nums, hit, tracer, result);
            tracer.pop();
            hit[i] =0;
        }
    }
}


#[cfg(test)]
mod test {
    use crate::other::algo_46::permute;

    #[test]
    fn test_permute() {
        let nums = vec![1, 2, 3];
        println!("{:?}", permute(nums));
    }
}