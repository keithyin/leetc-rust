
pub fn generate_pascals_trigle(num_rows: i32) -> Vec<Vec<i32>> {

    let mut results = vec![];

    results.push(vec![1]);

    for i in 1..num_rows {
        let pre = &results[i as usize - 1];
        let mut cur_row_res = vec![1];
        for pre_i in 0..pre.len()-1 {
            cur_row_res.push(pre[pre_i] + pre[pre_i + 1]);
        }
        cur_row_res.push(1);
        results.push(cur_row_res);
    }

    results
}