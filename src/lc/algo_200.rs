
fn dfs_200(grid: &mut Vec<Vec<char>>, cur_row: i32, cur_col: i32){

    let tot_row = grid.len() as i32;
    let tot_col = grid[0].len() as i32;

    grid[cur_row as usize][cur_col as usize] = '2';

    for (next_row, next_col) in [(cur_row+1, cur_col), (cur_row-1, cur_col),
        (cur_row, cur_col+1), (cur_row, cur_col-1)] {

        if next_row >= 0 && next_row < tot_row && next_col >= 0 && next_col < tot_col
            && grid[next_row as usize][next_col as usize] == '1' {

            dfs_200(grid, next_row, next_col);

        }

    }
}


pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '1' {
                result += 1;
                dfs_200(&mut grid, row as i32, col as i32);
            }
        }
    }
    result

}



