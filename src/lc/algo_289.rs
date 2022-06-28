

pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let tot_row = board.len();
    let tot_col = board[0].len();
    for row in 0..tot_row {
        for col in 0..tot_col {
            let cur_val = board[row][col];

            let mut zero_count = 0;
            let mut one_count = 0;

            for (surrounding_row, surrounding_col) in surrounding_coordinates(
                row as i32, col as i32, tot_row as i32, tot_col as i32) {
                if surrounding_row == -1 {
                    continue
                }
                let ori_v = get_origin_value(board[surrounding_row as usize][surrounding_col as usize]);
                if ori_v == 0 {
                    zero_count += 1;
                } else {
                    one_count +=1 ;
                }
            }

            if cur_val == 0 && one_count == 3{
                board[row][col] = -1;
            }
            if cur_val == 1 {
                if one_count < 2 || one_count > 3{
                    board[row][col] = -2;
                }
            }
        }
    }

    for row in 0..tot_row {
        for col in 0..tot_col {
            let cur_val = board[row][col];
            if cur_val == -1 {
                board[row][col] = 1;
            }
            if cur_val == -2 {
                board[row][col] = 0;
            }
        }
    }


}
// 0 -> -1, 1 -> -2
fn get_origin_value(val: i32) -> i32 {
    return if val >= 0 {
        val
    } else {
        val.abs() - 1
    };
}


fn surrounding_coordinates(row: i32, col: i32, tot_row: i32, tot_col: i32) -> Vec<(i32, i32)>{
    let mut coordinates = vec![];

    // up
    coordinates.push(if (row - 1) >= 0 {
        (row-1, col)
    } else {
        (-1, -1)
    });

    // down
    coordinates.push(if (row + 1) < tot_row {
        (row+1, col)
    } else {
        (-1, -1)
    });

    // left
    coordinates.push(if (col-1) >= 0 {
        (row, col-1)
    } else {
        (-1, -1)
    });

    // right
    coordinates.push(if (col+1) < tot_col {
        (row, col+1)
    } else {
        (-1, -1)
    });

    // up + left
    coordinates.push(if (row - 1) >= 0 && (col-1) >= 0 {
        (row-1, col-1)
    } else {
        (-1, -1)
    });

    // down + left
    coordinates.push(if (row + 1) < tot_row && (col-1)>=0{
        (row+1, col-1)
    } else {
        (-1, -1)
    });

    // up+right
    coordinates.push(if (col+1) >= 0 && (row-1)>=0{
        (row-1, col+1)
    } else {
        (-1, -1)
    });

    // down+right
    coordinates.push(if (col+1) < tot_col && (row+1) < tot_row{
        (row+1, col+1)
    } else {
        (-1, -1)
    });

    coordinates
}