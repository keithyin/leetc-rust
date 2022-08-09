use std::collections::HashSet;

pub fn is_point_valid(row_col: (usize, usize), occupied_point: &mut HashSet<(usize, usize)>,
                      occupied_col: &mut HashSet<usize>, queens: usize) -> bool{
    let row = row_col.0 as i32;
    let col = row_col.1 as i32;

    if occupied_col.contains(&(col as usize)) {
        return false;
    }

    for i in (-(queens as i32))..queens as i32 {
        let new_row = row - i;
        let new_col = col - i;
        let new_col2 = col + i;
        if new_row >= 0 && new_col >= 0 {
            if occupied_point.contains(&(new_row as usize, new_col as usize)) {
                return false;
            }
        }
        if new_row >= 0 && new_col2 >=0 {
            if occupied_point.contains(&(new_row as usize, new_col2 as usize)) {
                return false;
            }
        }
    }
    true
}

pub fn fmt_result(result: &Vec<HashSet<(usize, usize)>>, queens: usize) -> Vec<Vec<String>> {

    let mut res_str = vec![vec![vec![".".to_string(); queens]; queens]; result.len()];
    for i in 0..result.len() {

        let res_ref = &mut res_str[i];
        for (row, col) in result[i].iter() {
            res_ref[*row][*col] = "Q".to_string();
        }
    }

    res_str.iter()
        .map(|x| x.iter()
            .map(|x| x.join(""))
            .collect())
        .collect()
}

pub fn solve_n_queues_core(cur_row: usize, occupied_point: &mut HashSet<(usize, usize)>,
                           occupied_col: &mut HashSet<usize>, queens: usize, result: &mut usize) {
    if cur_row == queens {
        *result += 1;
        return;
    }


    for col in 0..queens {

        if is_point_valid((cur_row, col), occupied_point, occupied_col, queens) {

            occupied_point.insert((cur_row, col));
            occupied_col.insert(col);

            solve_n_queues_core(cur_row+1, occupied_point, occupied_col, queens, result);

            occupied_col.remove(&col);
            occupied_point.remove(&(cur_row, col));
        }

    }

}

pub fn total_n_queens(n: i32) -> i32{
    let mut occupied_point = HashSet::new();
    let mut occupied_col = HashSet::new();
    let mut result =0;
    solve_n_queues_core(0, &mut occupied_point, &mut occupied_col, n as usize, &mut result);
    result as i32
}

#[cfg(test)]
mod test {
    use crate::lc::algo_51::solve_n_queens;

    #[test]
    pub fn test() {

        println!("{:?}", solve_n_queens(4));
    }
}