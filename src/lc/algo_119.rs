
pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut pre = vec![1];
    for i in 1..=row_index {
        let mut cur = vec![1; i as usize + 1];
        for j in 1..(cur.len()-1) {
            cur[j] = pre[j-1] + pre[j];
        }
        pre = cur;
    }
    pre
}