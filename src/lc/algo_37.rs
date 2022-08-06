use std::collections::HashSet;


pub fn compute_block_idx(row: usize, col: usize) -> usize {
    let block_row = row / 3;
    let block_col = col / 3;
    block_row * 3 + block_col
}

pub fn compute_row_and_col(idx: usize) -> (usize, usize) {
    (idx / 9, idx % 9)
}

pub fn row_and_col_2_idx(row: usize, col: usize) -> usize {
    row * 9 + col
}

pub fn solve_sudoku_core(board: &mut Vec<Vec<char>>,
    row_exist: &mut Vec<HashSet<char>>,
    col_exist: &mut Vec<HashSet<char>>,
    block_exist: &mut Vec<HashSet<char>>, idx: usize) -> bool{

    if idx == 81 {
        return true;
    }

    let (row, col) = compute_row_and_col(idx);
    let block_idx = compute_block_idx(row, col);
    let cur_c = board[row][col];
    if cur_c == '.' {

        for c in ['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
            if !row_exist[row].contains(&c) && !col_exist[col].contains(&c) && !block_exist[block_idx].contains(&c) {
                row_exist[row].insert(c);
                col_exist[col].insert(c);
                block_exist[block_idx].insert(c);
                board[row][col] = c;
                let found = solve_sudoku_core(board, row_exist, col_exist, block_exist, idx+1);
                if found {
                    return true;
                }

                board[row][col] = '.';
                row_exist[row].remove(&c);
                col_exist[col].remove(&c);
                block_exist[block_idx].remove(&c);
            }
        }
    } else {
        return solve_sudoku_core(board, row_exist, col_exist, block_exist, idx + 1);
    }

    false

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
            let block_row = row / 3;
            let block_col = col / 3;
            let block_idx = block_row * 3 + block_col;


            row_exist[row].insert(cur_char);
            col_exist[col].insert(cur_char);
            block_exist[block_idx].insert(cur_char);

        }
    }

    solve_sudoku_core(board, &mut row_exist, &mut col_exist, &mut block_exist, 0);

}