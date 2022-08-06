use std::collections::{HashMap, HashSet};

pub fn map2vec(val: &HashMap<i32, usize>) -> Vec<i32> {
    let mut result = vec![];
    for (k, v) in val.iter() {
        for i in 0..*v {
            result.push(*k);
        }
    }
    result
}


pub fn combination_sum2_core(counter: &HashMap<i32, usize>,
                             distinct_vals: &Vec<i32>,
                             idx: usize,
                             residual: i32,
                             tracer: &mut HashMap<i32, usize>,
                             result: &mut Vec<Vec<i32>>) {

   if residual == 0 {
       result.push(map2vec(tracer));
       return;
   }

    if residual < 0 {
        return;
    }

    if idx == distinct_vals.len() {
        return;
    }
    let cur_val = distinct_vals[idx];
    for multiplier in 0..counter[&cur_val]+1 {
        let next_residual = residual - cur_val * (multiplier as i32);

        tracer.insert(cur_val, multiplier);
        combination_sum2_core(counter, distinct_vals, idx + 1, next_residual, tracer, result);
        tracer.remove(&cur_val);
    }

}

pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort();
    let mut tracer = HashMap::new();
    let mut result = vec![];
    let mut counter = HashMap::new();
    let mut distinct_vals = HashSet::new();

    for v in candidates.iter() {
        distinct_vals.insert(*v);
        if let Some(num) = counter.get_mut(v) {
            *num += 1;
        } else {
            counter.insert(*v, 1 as usize);
        }
    }

    combination_sum2_core(&counter, &distinct_vals.iter().map(|x| *x).collect(), 0, target, &mut tracer, &mut result);

    result


}

#[cfg(test)]
mod test {
    use crate::lc::algo_40::combination_sum2;

    #[test]
    pub fn test_comb() {

        let candidates = vec![2,5,2,1,2];
        let target = 5;
        println!("{:?}", combination_sum2(candidates, target));
    }
}