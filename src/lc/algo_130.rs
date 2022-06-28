
fn dfs_algo_130(board: &mut Vec<Vec<char>>, cur_row: usize, cur_col: usize, o_to_what: char) {
    let tot_row = board.len() as i32;
    let tot_col = board[0].len() as i32;

    board[cur_row][cur_col] = o_to_what;

    let cur_row = cur_row as i32;
    let cur_col  = cur_col as i32;
    for (next_row, next_col) in [
        (cur_row+1, cur_col), (cur_row-1, cur_col), (cur_row, cur_col+1), (cur_row, cur_col-1)] {

        if next_row >= 0 && next_row < tot_row && next_col >= 0 && next_col < tot_col && board[next_row as usize][next_col as usize] == 'O' {
            dfs_algo_130(board, next_row as usize, next_col as usize, o_to_what);
        }
    }

}

pub fn surrounded_regions(board: &mut Vec<Vec<char>>) {
    let tot_row = board.len();
    let tot_col = board[0].len();
    // 先处理边缘的
    // top 边
    for col in 0..tot_col {
        if board[0][col] == 'O' {
            dfs_algo_130(board, 0, col, '-');
        }
        if board[tot_row-1][col] == 'O' {
            dfs_algo_130(board, tot_row-1, col, '-');
        }
    }

    for row in 0..tot_row {
        if board[row][0] == 'O' {
            dfs_algo_130(board, row, 0, '-');
        }
        if board[row][tot_col-1] == 'O' {
            dfs_algo_130(board, row, tot_col-1, '-');
        }
    }

    for cur_row in 1..tot_row {
        for cur_col in 1..tot_col {
            if board[cur_row][cur_col] == 'O' {
                dfs_algo_130(board, cur_row, cur_col, 'X');
            }
        }
    }

    for cur_row in 0..tot_row {
        for cur_col in 0..tot_col {
            if board[cur_row][cur_col] == '-' {
                board[cur_row][cur_col] = 'O';
            }
        }
    }

}

