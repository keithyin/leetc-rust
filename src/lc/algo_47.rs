use std::collections::HashMap;

pub fn permute_unique_core(counter: &mut HashMap<i32, usize>, residual: usize, tracer: &mut Vec<(i32, usize)>, result: &mut Vec<Vec<i32>>, last_choice: i32) {
    if residual == 0 {
        let mut cur_res = vec![];
        for (v, num) in tracer.iter() {
            for _ in 0..*num {
                cur_res.push(*v);
            }
        }
        result.push(cur_res);
        return;
    }

    let distinct_vals:  Vec<i32>= counter.iter()
        .filter(|(k,v )| **k != last_choice && **v > 0)
        .map(|(k, v)| *k)
        .collect();

    for v in distinct_vals.iter() {

        let max_num = counter[v];
        for choose_num in 1..max_num+1 {

            tracer.push((*v, choose_num));
            counter.insert(*v, max_num - choose_num);
            permute_unique_core(counter, residual - choose_num, tracer, result, *v);

            counter.insert(*v, max_num);
            tracer.pop();
        }
    }

}


pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {

    let mut counter = HashMap::new();
    let mut tracer = vec![];
    let mut result = vec![];

    for v in nums.iter() {
        if let Some(c) = counter.get_mut(v) {
            *c += 1;
        } else {
            counter.insert(*v, 1 as usize);
        }
    }

    permute_unique_core(&mut counter, nums.len(), &mut tracer, &mut result, 100);

    result
}

#[cfg(test)]
mod test {
    use crate::lc::algo_47::permute_unique;

    #[test]
    pub fn test() {
        let nums = vec![1,1,2];
        println!("{:?}", permute_unique(nums));
    }
}