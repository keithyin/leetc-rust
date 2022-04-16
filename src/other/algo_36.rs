use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {

    let candidates: HashSet<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'].into_iter().collect();

    let mut board = board;
    let board_len = 9;
    let mut rows_info: Vec<HashSet<char>> = vec![HashSet::new(); board_len];
    let mut cols_info:Vec<HashSet<char>>  = vec![HashSet::new(); board_len];
    let mut blocks_info:Vec<HashSet<char>>  = vec![HashSet::new(); board_len];
    for row in 0..board_len {
        for col in 0..board_len {

            if board[row][col] == '.' {
                continue;
            }

            if rows_info[row].contains(&board[row][col]) {
                return false;
            }

            rows_info[row].insert(board[row][col]);

            if cols_info[col].contains(&board[row][col]) {
                return false;
            }
            cols_info[col].insert(board[row][col]);

            let block_idx = get_block_idx(row, col);
            if blocks_info[block_idx].contains(&board[row][col]) {
                return false;
            }
            blocks_info[block_idx].insert(board[row][col]);
        }
    }

    return is_valid_sudoku_core(&mut board, &mut rows_info, &mut cols_info, &mut blocks_info, 0, &candidates);
}

fn is_valid_sudoku_core(board: &mut Vec<Vec<char>>,
                        rows_info: &mut Vec<HashSet<char>>,
                        cols_info: &mut Vec<HashSet<char>>,
                        blocks_info: &mut Vec<HashSet<char>>,
                        pos: usize,
                        candidates: &HashSet<char>
) -> bool{

    if pos == 81 {
        return true;
    }
    let (row, col) = pos_2_row_col(pos);
    if board[row][col] != '.' {
        return is_valid_sudoku_core(board, rows_info, cols_info, blocks_info, pos+1, candidates);
    }

    let block_idx = get_block_idx(row, col);

    let row_col_info_union:HashSet<char> = rows_info[row].union(&cols_info[col]).map(|x| *x).collect();
    let row_col_block_union: HashSet<char> = row_col_info_union.union(&blocks_info[block_idx]).map(|x|*x).collect();

    let remained_candidates:HashSet<char> = candidates.difference(&row_col_block_union).map(|x| *x).collect();
    for candidate in remained_candidates.iter() {
        board[row][col] = *candidate;
        rows_info[row].insert(*candidate);
        cols_info[col].insert(*candidate);
        blocks_info[block_idx].insert(*candidate);

        let valid = is_valid_sudoku_core(board, rows_info, cols_info, blocks_info, pos+1, candidates);
        if valid  {
            return true;
        }
        rows_info[row].remove(candidate);
        cols_info[col].remove(candidate);
        blocks_info[block_idx].remove(candidate);
        board[row][col] = '.';
    }

    false

}

pub fn pos_2_row_col(pos: usize) -> (usize, usize){
    let row = pos / 9;
    let col = pos - row * 9;
    (row, col)
}

pub fn get_block_idx(row: usize, col: usize) -> usize {
    return (row / 3) as usize * 3 + col / 3 as usize
}

#[cfg(test)]
mod test {
    use crate::other::algo_36::{get_block_idx, pos_2_row_col};

    #[test]
    fn test_is_valid_sudoku() {

    }

    #[test]
    fn test_get_block_idx() {
        println!("{}", get_block_idx(8, 8))
    }

    #[test]
    fn test_pos_2_row_col() {
        println!("{:?}", pos_2_row_col(80));
    }
}