
pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let tot_row = matrix.len();
    let tot_col = matrix[0].len();

    let mut visited = vec![vec![0; tot_col]; tot_row];

    let mut longest = i32::MIN;

    for row in 0..tot_row {
        for col in 0..tot_col {
            let from_this_point_len = dfs(&matrix, &mut visited, row as i32, col as i32);
            longest = if longest < from_this_point_len {
                from_this_point_len
            } else {
                longest
            };
        }
    }

    longest
}

fn get_coordinates(
    cur_row: i32, cur_col: i32, tot_row: i32, tot_col: i32, orientation: String) -> (i32, i32) {

    match orientation.as_str() {
        "up" => {
            if (cur_row - 1) >= 0 {
                (cur_row - 1, cur_col)
            } else {
                (-1, -1)
            }

        },
        "down" => {
            if (cur_row+1) < tot_row {
                (cur_row + 1, cur_col)
            } else {
                (-1, -1)
            }
        },
        "left" => {
            if (cur_col - 1) >= 0 {
                (cur_row, cur_col - 1)
            } else {
                (-1, -1)
            }
        },
        "right" => {
            if (cur_col + 1) < tot_col {
                (cur_row, cur_col + 1)
            } else {
                (-1, -1)
            }
        },
        _ => panic!("invalid orientation")
    }
}

fn dfs(matrix: &Vec<Vec<i32>>, visited: &mut Vec<Vec<i32>>, row: i32, col: i32) -> i32 {

    visited[row as usize][col as usize] = 1;
    let mut max_len_from_here = 0;

    let (up_row, up_col) = get_coordinates(row, col,
        matrix.len() as i32, matrix[0].len() as i32, "up".to_string());
    if up_row >= 0 && visited[up_row as usize][up_col as usize] == 0 && matrix[row as usize][col as usize] < matrix[up_row as usize][up_col as usize] {
        max_len_from_here += dfs(matrix, visited, up_row, up_col);
    }

    let (down_row, down_col) = get_coordinates(
        row, col,matrix.len() as i32, matrix[0].len() as i32,
        "down".to_string());

    if down_col >= 0 && visited[down_row as usize][down_col as usize] == 0 && matrix[row as usize][col as usize] < matrix[down_row as usize][down_col as usize] {
        let down_len = dfs(matrix, visited, down_row, down_col);
        max_len_from_here = if max_len_from_here > down_len {
            max_len_from_here
        } else {
            down_len
        };
    }

    let (left_row, left_col) = get_coordinates(
        row, col,matrix.len() as i32, matrix[0].len() as i32,
        "left".to_string());

    if left_row >= 0 && visited[left_row as usize][left_col as usize] == 0 && matrix[row as usize][col as usize] < matrix[left_row as usize][left_col as usize] {
        let left_len = dfs(matrix, visited, left_row, left_col);
        max_len_from_here = if max_len_from_here > left_len {
            max_len_from_here
        } else {
            left_len
        };
    }

    let (right_row, right_col) = get_coordinates(
        row, col,matrix.len() as i32, matrix[0].len() as i32,
        "right".to_string());

    if right_row >= 0 && visited[right_row as usize][right_col as usize] == 0 && matrix[row as usize][col as usize] < matrix[right_row as usize][right_col as usize] {
        let right_len = dfs(matrix, visited, right_row, right_col);
        max_len_from_here = if max_len_from_here > right_len {
            max_len_from_here
        } else {
            right_len
        };
    }



    visited[row as usize][col as usize] = 0;

    max_len_from_here + 1
}