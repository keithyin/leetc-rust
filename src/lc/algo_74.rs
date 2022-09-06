
pub fn idx2rowcol(idx: usize, tot_row: usize, tot_col: usize) -> (usize, usize) {

    (idx / tot_col, idx % tot_col)

}

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let tot_row = matrix.len();
    let tot_col = matrix[0].len();

    let mut begin = 0 as i32;
    let mut end = (tot_row * tot_col) as i32;
    while begin < end {
        let mid = ((begin + end) / 2) as usize;
        let row_col = idx2rowcol(mid, tot_row, tot_col);
        if matrix[row_col.0][row_col.1] == target {
            return true;
        } else if matrix[row_col.0][row_col.1] < target {
            begin = mid as i32 + 1;
        } else {
            end = mid as i32;
        }

    }

    false


}