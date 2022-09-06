

pub fn combine_core(n: i32, i: i32, left: i32, tracer: &mut Vec<i32>, reslut: &mut Vec<Vec<i32>>) {
    if left == 0 {
        reslut.push(tracer.clone());
        return;
    }
    if i > n {
        return;
    }

    tracer[(k - left) as usize] = i;
    combine_core(n, i+1, left-1, tracer, reslut);
    combine_core(n, i+1, left, tracer, reslut);

}

pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut tracer = vec![0; k as usize];
    combine_core(n, 1, k, &mut tracer, &mut res);
    res
}