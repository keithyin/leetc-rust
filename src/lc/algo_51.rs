use std::collections::HashSet;

pub fn is_point_valid(row_col: (usize, usize), occupied_point: HashSet<(usize, usize)>,
                      occupied_col: HashSet<usize>, queens: usize) -> bool{
    let row = row_col.0 as i32;
    let col = row_col.1 as i32;

    if occupied_col.contains(&(col as usize)) {
        return false;
    }

    for i in (-queens as i32)..queens as i32 {
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


pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {

    vec![]
}