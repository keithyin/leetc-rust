

pub fn combination_sum(candidates: Vec<i32>, target: i32, tracer: &mut Vec<i32>,
                       idx: usize, result: &mut Vec<Vec<i32>>) {

    if target == 0 {
        result.push(tracer.clone());
    }

    let cur_candidate = candidates[idx];
    let mut residual = target;
    while residual >= 0 {
        residual -= cur_candidate;
        tracer.push(cur_candidate);
    }
}