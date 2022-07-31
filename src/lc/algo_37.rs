use std::collections::HashSet;


pub fn compute_block_idx(row: usize, col: usize) -> usize {
    let block_row = row / 3;
    let block_col = col / 3;
    block_row * 3 + block_col
}

pub fn solve_sudoku_core(board: &mut Vec<Vec<char>>, row_exist: &mut Vec<HashSet<char>>, col_exist: &mut Vec<HashSet<char>>,
    block_exist: Vec<HashSet<char>>, row: usize, col: usize) {

}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {


    let mut row_exist = vec![HashSet::new(); 9];
    let mut col_exist = vec![HashSet::new(); 9];
    let mut block_exist = vec![HashSet::new(); 9];

    for row in 0..9 {
        for col in 0..9 {
            let cur_char = board[row][col];
            if cur_char == '.' {
                continue;
            }
            if row_exist[row].contains(&cur_char) {
                return false;
            }
            if col_exist[col].contains(&cur_char) {
                return false;
            }

            let block_row = row / 3;
            let block_col = col / 3;
            let block_idx = block_row * 3 + block_col;
            if block_exist[block_idx].contains(&cur_char) {
                return false;
            }

            row_exist[row].insert(cur_char);
            col_exist[col].insert(cur_char);
            block_exist[block_idx].insert(cur_char);

        }
    }

    true

}